use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use tokio::{sync::oneshot, task::JoinSet};
use tracing::info;
use yixuan_logic::debug::GMCmd;
use yixuan_proto::{
    head::PacketHead,
    server_only::{PlayerData, PlayerDataChangedNotify},
};
use yixuan_service::{
    ConfigurableServiceModule, ServiceContext, Startable, config::ServiceType,
    network::client::NetworkClient,
};

use crate::{config::ClusterConfig, handlers::NotifyQueue, resources::NapResources};

mod logic_loop;

pub struct PlayerCommandResult {
    pub notifies: Vec<(u16, Vec<u8>)>,
    pub response: Option<(u16, Vec<u8>)>,
}

#[derive(Clone)]
pub struct PlayerLogicCluster {
    #[expect(dead_code)]
    pub id: u32,
    command_tx: mpsc::Sender<ClusterCommand>,
}

pub struct PlayerLogicClusterManager {
    cluster_map: HashMap<u32, PlayerLogicCluster>,
    cluster_load_map: Arc<HashMap<u32, AtomicUsize>>,
}

pub struct LogicClusterConfig {
    pub cluster: ClusterConfig,
    pub resources: &'static NapResources,
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
        result_awaiter_tx: oneshot::Sender<NotifyQueue>,
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

impl Startable for PlayerLogicClusterManager {
    fn start(
        &self,
        service: Arc<yixuan_service::ServiceContext>,
    ) -> impl Future<Output = ()> + Send + Sync {
        let (report_tx, report_rx) = tokio::sync::mpsc::channel(self.cluster_map.len());
        let (player_update_tx, player_update_rx) =
            tokio::sync::mpsc::channel(self.cluster_map.len() * 16);

        self.cluster_map.iter().for_each(|(_, cluster)| {
            cluster.set_report_listener(report_tx.clone());
            cluster.set_player_update_listener(player_update_tx.clone());
        });

        let mut join_set = JoinSet::new();
        join_set.spawn(Self::player_update_receiver_loop(player_update_rx, service));
        join_set.spawn(Self::report_receiver_loop(
            report_rx,
            Arc::clone(&self.cluster_load_map),
        ));

        async {
            let _ = join_set.join_all().await;
        }
    }
}

impl ConfigurableServiceModule for PlayerLogicClusterManager {
    type Config = LogicClusterConfig;

    fn new(_context: &yixuan_service::ServiceContext, config: Self::Config) -> Self {
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
        thread::spawn(move || logic_loop::logic_loop(id, resources, rx));

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

    pub async fn push_gm_command(&self, player_uid: u32, cmd: GMCmd) -> NotifyQueue {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(ClusterCommand::PushGmCommand { player_uid, cmd, result_awaiter_tx: tx })
            .unwrap();

        rx.await.unwrap()
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
}
