use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use tokio::sync::oneshot;
use tracing::info;
use vivian_logic::{GameState, debug::GMCmd};
use vivian_proto::{
    head::PacketHead,
    server_only::{PlayerData, PlayerDataChangedNotify},
};
use vivian_service::{
    ConfigurableServiceModule, ServiceContext, Startable, config::ServiceType,
    network::client::NetworkClient,
};

use crate::{
    config::ClusterConfig,
    handlers::NetContext,
    player::{ModelManager, Player},
    resources::NapResources,
    util::gm_util,
};

pub struct PlayerCommandResult {
    pub notifies: Vec<(u16, Vec<u8>)>,
    pub response: Option<(u16, Vec<u8>)>,
}

enum ClusterCommand {
    SetReportListener(tokio::sync::mpsc::Sender<ClusterPerformanceReport>),
    SetPlayerUpdateListener(tokio::sync::mpsc::Sender<PlayerUpdate>),
    CreateSlot {
        player_uid: u32,
        player_data: Box<PlayerData>,
    },
    PushPlayerCommand {
        player_uid: u32,
        cmd_id: u16,
        body: Vec<u8>,
        result_awaiter_tx: oneshot::Sender<PlayerCommandResult>,
    },
    PushGmCommand {
        player_uid: u32,
        cmd: GMCmd,
    },
    RemovePlayer {
        player_uid: u32,
    },
}

#[derive(Debug)]
struct ClusterPerformanceReport {
    cluster_id: u32,
    player_slots: usize,
}

struct PlayerUpdate {
    uid: u32,
    data: PlayerData,
}

#[derive(Clone)]
pub struct PlayerLogicCluster {
    #[expect(dead_code)]
    pub id: u32,
    command_tx: mpsc::Sender<ClusterCommand>,
}

pub struct PlayerSlot {
    pub player: Player,
    pub game_state: Option<GameState>,
}

pub struct PlayerLogicClusterManager {
    cluster_map: HashMap<u32, PlayerLogicCluster>,
    cluster_load_map: Arc<HashMap<u32, AtomicUsize>>,
}

pub struct LogicClusterConfig {
    pub cluster: ClusterConfig,
    pub resources: &'static NapResources,
}

impl Startable for PlayerLogicClusterManager {
    fn start(
        &self,
        service: Arc<vivian_service::ServiceContext>,
    ) -> impl Future<Output = ()> + Send + Sync {
        let (report_tx, report_rx) = tokio::sync::mpsc::channel(self.cluster_map.len());
        let (player_update_tx, player_update_rx) =
            tokio::sync::mpsc::channel(self.cluster_map.len() * 16);

        self.cluster_map.iter().for_each(|(_, cluster)| {
            cluster.set_report_listener(report_tx.clone());
            cluster.set_player_update_listener(player_update_tx.clone());
        });

        tokio::spawn(Self::player_update_receiver_loop(player_update_rx, service));

        Self::report_receiver_loop(report_rx, Arc::clone(&self.cluster_load_map))
    }
}

impl ConfigurableServiceModule for PlayerLogicClusterManager {
    type Config = LogicClusterConfig;

    fn new(_context: &vivian_service::ServiceContext, config: Self::Config) -> Self {
        Self::new(config.cluster.num_clusters, config.resources)
    }
}

impl PlayerLogicClusterManager {
    pub fn new(num_clusters: u32, resources: &'static NapResources) -> Self {
        Self {
            cluster_map: (1..=num_clusters)
                .map(|id| (id, PlayerLogicCluster::spawn(id, resources)))
                .collect(),
            cluster_load_map: Arc::new(
                (1..=num_clusters)
                    .map(|id| (id, AtomicUsize::new(0)))
                    .collect(),
            ),
        }
    }

    pub fn get_least_loaded_cluster(&self) -> PlayerLogicCluster {
        let id = *self
            .cluster_load_map
            .iter()
            .min_by_key(|(_, load)| load.load(Ordering::Relaxed))
            .unwrap()
            .0;

        self.get_cluster(id).unwrap()
    }

    pub fn get_cluster(&self, id: u32) -> Option<PlayerLogicCluster> {
        self.cluster_map.get(&id).cloned()
    }

    async fn report_receiver_loop(
        mut report_rx: tokio::sync::mpsc::Receiver<ClusterPerformanceReport>,
        load_map: Arc<HashMap<u32, AtomicUsize>>,
    ) {
        while let Some(performance_report) = report_rx.recv().await {
            if let Some(load) = load_map.get(&performance_report.cluster_id) {
                load.store(performance_report.player_slots, Ordering::Relaxed);
                info!("[CLUSTER] {performance_report:?}");
            }
        }
    }

    async fn player_update_receiver_loop(
        mut player_update_rx: tokio::sync::mpsc::Receiver<PlayerUpdate>,
        service: Arc<ServiceContext>,
    ) {
        while let Some(player_update) = player_update_rx.recv().await {
            let client = service.resolve::<NetworkClient>();
            client
                .send_notify(
                    ServiceType::Dbgate,
                    PacketHead::default(),
                    PlayerDataChangedNotify {
                        player_uid: player_update.uid,
                        player_data: Some(player_update.data),
                    },
                )
                .await;
        }
    }
}

impl PlayerLogicCluster {
    pub fn spawn(id: u32, resources: &'static NapResources) -> Self {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || Self::logic_loop(id, resources, rx));

        Self { id, command_tx: tx }
    }

    pub fn create_player_slot(&self, uid: u32, data: PlayerData) {
        self.command_tx
            .send(ClusterCommand::CreateSlot {
                player_uid: uid,
                player_data: Box::new(data),
            })
            .unwrap();
    }

    pub fn remove_player(&self, uid: u32) {
        self.command_tx
            .send(ClusterCommand::RemovePlayer { player_uid: uid })
            .unwrap();
    }

    pub async fn handle_player_command(
        &self,
        uid: u32,
        cmd_id: u16,
        body: Vec<u8>,
    ) -> PlayerCommandResult {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(ClusterCommand::PushPlayerCommand {
                player_uid: uid,
                cmd_id,
                body,
                result_awaiter_tx: tx,
            })
            .unwrap();

        rx.await.unwrap()
    }

    pub fn push_gm_command(&self, player_uid: u32, cmd: GMCmd) {
        self.command_tx
            .send(ClusterCommand::PushGmCommand { player_uid, cmd })
            .unwrap();
    }

    fn set_report_listener(&self, tx: tokio::sync::mpsc::Sender<ClusterPerformanceReport>) {
        self.command_tx
            .send(ClusterCommand::SetReportListener(tx))
            .unwrap();
    }

    fn set_player_update_listener(&self, tx: tokio::sync::mpsc::Sender<PlayerUpdate>) {
        self.command_tx
            .send(ClusterCommand::SetPlayerUpdateListener(tx))
            .unwrap();
    }

    fn logic_loop(
        cluster_id: u32,
        resources: &'static NapResources,
        command_rx: mpsc::Receiver<ClusterCommand>,
    ) {
        let mut slots = HashMap::new();

        let mut report_tx = None;
        let mut player_update_tx = None;

        while let Ok(command) = command_rx.recv() {
            match command {
                ClusterCommand::SetReportListener(tx) => {
                    report_tx = Some(tx);
                }
                ClusterCommand::SetPlayerUpdateListener(tx) => {
                    player_update_tx = Some(tx);
                }
                ClusterCommand::CreateSlot {
                    player_uid,
                    player_data,
                } => {
                    let mut player = Player::load_from_pb(player_uid, *player_data, resources);
                    player.on_login();

                    slots.insert(
                        player_uid,
                        PlayerSlot {
                            player,
                            game_state: None,
                        },
                    );

                    report_tx.as_ref().inspect(|tx| {
                        let _ = tx.blocking_send(ClusterPerformanceReport {
                            cluster_id,
                            player_slots: slots.len(),
                        });
                    });
                }
                ClusterCommand::RemovePlayer { player_uid } => {
                    if let Some(mut slot) = slots.remove(&player_uid) {
                        if let Some(player_update_tx) = player_update_tx.as_ref() {
                            if let Some(mut state) = slot.game_state {
                                slot.player.save_scene_snapshot(&mut state);
                                let data = slot.player.build_full_update();

                                let _ = player_update_tx.blocking_send(PlayerUpdate {
                                    uid: slot.player.uid,
                                    data,
                                });
                            }
                        }

                        report_tx.as_ref().inspect(|tx| {
                            let _ = tx.blocking_send(ClusterPerformanceReport {
                                cluster_id,
                                player_slots: slots.len(),
                            });
                        });
                    }
                }
                ClusterCommand::PushPlayerCommand {
                    player_uid,
                    cmd_id,
                    body,
                    result_awaiter_tx,
                } => {
                    if let Some(slot) = slots.get_mut(&player_uid) {
                        let mut context =
                            NetContext::new(&mut slot.player, &mut slot.game_state, resources);

                        super::handlers::handle_command(&mut context, cmd_id, body);

                        if context.player.loading_finished()
                            && context.player.has_models_to_synchronize()
                        {
                            context
                                .notifies
                                .prepend_notify(context.player.build_player_sync_notify());

                            // TODO: more generic way to send notifies from models?
                            context
                                .player
                                .avatar_model
                                .send_add_avatar_notify(&mut context.notifies);
                        }

                        if let Some(state) = context.game_state.as_mut() {
                            state.flush_notifies(&mut context.notifies);
                        }

                        let _ = result_awaiter_tx.send(PlayerCommandResult {
                            notifies: context.notifies.drain(),
                            response: context.response,
                        });

                        if let Some(player_update_tx) = player_update_tx.as_ref() {
                            if slot.player.is_any_model_modified() {
                                let data = slot.player.build_partial_update();
                                let _ = player_update_tx.blocking_send(PlayerUpdate {
                                    uid: slot.player.uid,
                                    data,
                                });

                                slot.player.changes_acknowledged();
                            }
                        }
                    }
                }
                ClusterCommand::PushGmCommand { player_uid, cmd } => {
                    if let Some(slot) = slots.get_mut(&player_uid) {
                        gm_util::execute_gm_cmd(&mut slot.player, cmd);
                    }
                }
            }
        }
    }
}
