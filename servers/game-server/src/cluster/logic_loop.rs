use std::collections::HashMap;

use tokio::sync::mpsc;
use yixuan_logic::GameState;

use crate::{
    handlers::{NetContext, NotifyQueue},
    player::{ModelManager, Player},
    resources::NapResources,
    util,
};

use super::{ClusterCommand, ClusterPerformanceReport, PlayerCommandResult, PlayerUpdate};

#[derive(Default)]
struct ClusterState {
    cluster_id: u32,
    slots: HashMap<u32, PlayerSlot>,
    report_tx: Option<mpsc::Sender<ClusterPerformanceReport>>,
    player_update_tx: Option<tokio::sync::mpsc::Sender<PlayerUpdate>>,
}

struct PlayerSlot {
    player: Player,
    game_state: Option<GameState>,
}

pub fn logic_loop(
    cluster_id: u32,
    resources: &'static NapResources,
    command_rx: std::sync::mpsc::Receiver<ClusterCommand>,
) {
    let mut state = ClusterState {
        cluster_id,
        ..Default::default()
    };

    while let Ok(command) = command_rx.recv() {
        match command {
            ClusterCommand::SetReportListener(tx) => {
                state.report_tx = Some(tx);
            }
            ClusterCommand::SetPlayerUpdateListener(tx) => {
                state.player_update_tx = Some(tx);
            }
            ClusterCommand::CreateSlot {
                player_uid,
                player_data,
            } => {
                let mut player = Player::load_from_pb(player_uid, *player_data, resources);
                player.on_login();

                state.slots.insert(
                    player_uid,
                    PlayerSlot {
                        player,
                        game_state: None,
                    },
                );

                send_performance_report(&state);
            }
            ClusterCommand::RemovePlayer { player_uid } => {
                if let Some(mut slot) = state.slots.remove(&player_uid) {
                    if let Some(player_update_tx) = state.player_update_tx.as_ref() {
                        if let Some(mut state) = slot.game_state {
                            slot.player.save_scene_snapshot(&mut state);
                            let data = slot.player.build_full_update();

                            let _ = player_update_tx.blocking_send(PlayerUpdate {
                                uid: slot.player.uid,
                                data,
                            });
                        }
                    }

                    send_performance_report(&state);
                }
            }
            ClusterCommand::PushPlayerCommand {
                player_uid,
                cmd_id,
                body,
                result_awaiter_tx,
            } => {
                if let Some(slot) = state.slots.get_mut(&player_uid) {
                    let mut context =
                        NetContext::new(&mut slot.player, &mut slot.game_state, resources);

                    crate::handlers::handle_command(&mut context, cmd_id, body);

                    enqueue_player_notifies(
                        context.player,
                        context.game_state.as_mut(),
                        &mut context.notifies,
                    );

                    let _ = result_awaiter_tx.send(PlayerCommandResult {
                        notifies: context.notifies.drain(),
                        response: context.response,
                    });

                    if let Some(player_update_tx) = state.player_update_tx.as_ref() {
                        flush_changes_to_dbgate(&mut slot.player, player_update_tx);
                    }
                }
            }
            ClusterCommand::PushGmCommand {
                player_uid,
                cmd,
                result_awaiter_tx,
            } => {
                if let Some(slot) = state.slots.get_mut(&player_uid) {
                    util::gm_util::execute_gm_cmd(&mut slot.player, slot.game_state.as_mut(), cmd);

                    let mut queue = NotifyQueue::default();
                    enqueue_player_notifies(&slot.player, slot.game_state.as_mut(), &mut queue);

                    let _ = result_awaiter_tx.send(queue);

                    if let Some(player_update_tx) = state.player_update_tx.as_ref() {
                        flush_changes_to_dbgate(&mut slot.player, player_update_tx);
                    }
                }
            }
        }
    }
}

fn enqueue_player_notifies(
    player: &Player,
    state: Option<&mut GameState>,
    queue: &mut NotifyQueue,
) {
    if player.loading_finished() && player.has_models_to_synchronize() {
        queue.prepend_notify(player.build_player_sync_notify());

        // TODO: more generic way to send notifies from models?
        player.avatar_model.send_add_avatar_notify(queue);
    }

    if let Some(state) = state {
        state.flush_notifies(queue);
    }
}

fn flush_changes_to_dbgate(player: &mut Player, player_update_tx: &mpsc::Sender<PlayerUpdate>) {
    if player.is_any_model_modified() {
        let data = player.build_partial_update();
        let _ = player_update_tx.blocking_send(PlayerUpdate {
            uid: player.uid,
            data,
        });

        player.changes_acknowledged();
    }
}

fn send_performance_report(state: &ClusterState) {
    state.report_tx.as_ref().inspect(|tx| {
        let _ = tx.blocking_send(ClusterPerformanceReport {
            cluster_id: state.cluster_id,
            player_slots: state.slots.len(),
        });
    });
}
