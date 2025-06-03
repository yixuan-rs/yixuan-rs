use std::sync::Arc;

use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, warn};
use yixuan_proto::{
    NetCmd, PlayerGetTokenCsReq, PlayerGetTokenScRsp,
    head::PacketHead,
    server_only::{PlayerData, PlayerDataChangedNotify, PlayerGetDataReq, PlayerGetDataRsp},
};
use yixuan_service::{
    ServiceContext, ServiceScope,
    network::{
        NetworkEntityManager, NetworkEventListener,
        entity::NetworkEntity,
        packet::{GetProtoError, NetPacket},
    },
};

use crate::{
    database::{BinaryDataFetchError, DbConnection},
    sdk::TokenVerificationModule,
};

struct NetworkListener(mpsc::UnboundedSender<(u64, NetPacket)>);

impl NetworkEventListener for NetworkListener {
    fn on_receive(&self, entity_id: u64, packet: NetPacket) {
        self.0.send((entity_id, packet)).unwrap();
    }

    fn on_disconnect(&self, _entity_id: u64) {}
}

pub fn start_handler_task() -> (
    oneshot::Sender<Arc<ServiceContext>>,
    impl NetworkEventListener,
) {
    let (sv_tx, sv_rx) = oneshot::channel();
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(handler_loop(sv_rx, rx));
    (sv_tx, NetworkListener(tx))
}

async fn handler_loop(
    lazy_service: oneshot::Receiver<Arc<ServiceContext>>,
    mut rx: mpsc::UnboundedReceiver<(u64, NetPacket)>,
) {
    let service = lazy_service.await.unwrap();

    while let Some((id, packet)) = rx.recv().await {
        tokio::spawn(process_cmd(Arc::clone(&service), id, packet));
    }
}

async fn process_cmd(service: Arc<ServiceContext>, entity_id: u64, packet: NetPacket) {
    if let Some(entity) = service.resolve::<NetworkEntityManager>().get(entity_id) {
        let scope = service.new_scope().with_variable(entity).build();
        if let Err(err) = handle_cmd(scope.as_ref(), packet).await {
            error!("failed to decode client cmd: {err}");
        }
    } else {
        warn!("process_cmd: client entity with id {entity_id} not found");
    }
}

async fn handle_cmd(scope: &ServiceScope, packet: NetPacket) -> Result<(), GetProtoError> {
    debug!("received: {} {:?}", packet.cmd_id, packet.head);

    match packet.cmd_id {
        PlayerGetTokenCsReq::CMD_ID => {
            handle_player_get_token(scope, packet.head, packet.get_proto()?).await
        }
        PlayerGetDataReq::CMD_ID => {
            handle_player_get_data(scope, packet.head, packet.get_proto()?).await
        }
        PlayerDataChangedNotify::CMD_ID => {
            handle_player_data_changed(scope, packet.head, packet.get_proto()?).await
        }
        _ => warn!("unhandled: {}", packet.cmd_id),
    }

    Ok(())
}

async fn handle_player_get_token(
    scope: &ServiceScope,
    head: PacketHead,
    request: PlayerGetTokenCsReq,
) {
    debug!("{request:?}");

    let token_verification_module = scope.resolve::<TokenVerificationModule>();
    let (retcode, uid) = if !token_verification_module
        .verify(&request.account_uid, &request.token)
        .await
    {
        error!(
            "SDK account verification failed! (account_uid: {})",
            request.account_uid
        );

        (1007, 0)
    } else {
        let db = scope.resolve::<DbConnection>();
        let uid = db
            .fetch_uid_for_account(&request.account_uid)
            .await
            .expect("fetch_uid_for_account failed") as u32;

        (0, uid)
    };

    let response = PlayerGetTokenScRsp {
        retcode,
        uid,
        ..Default::default()
    };

    let entity = scope.fetch::<Arc<NetworkEntity>>().unwrap();
    entity.send(NetPacket::new(
        PacketHead {
            session_id: head.session_id,
            player_uid: head.player_uid,
            ack_packet_id: head.packet_id,
            ..Default::default()
        },
        response,
    ));
}

async fn handle_player_get_data(scope: &ServiceScope, head: PacketHead, request: PlayerGetDataReq) {
    let uid = request.player_uid as i32;
    let db = scope.resolve::<DbConnection>();

    let player_data = match fetch_player_data(db, uid).await {
        Ok(player_data) => player_data,
        Err(err) => {
            error!("failed to fetch player data for uid {uid}, error: {err}");
            return;
        }
    };

    let entity = scope.fetch::<Arc<NetworkEntity>>().unwrap();

    entity.send(NetPacket::new(
        PacketHead {
            player_uid: head.player_uid,
            session_id: head.session_id,
            ack_packet_id: head.packet_id,
            ..Default::default()
        },
        PlayerGetDataRsp {
            retcode: 0,
            player_data: Some(player_data),
        },
    ));
}

async fn handle_player_data_changed(
    scope: &ServiceScope,
    _: PacketHead,
    notify: PlayerDataChangedNotify,
) {
    let db = scope.resolve::<DbConnection>();

    if let Err(err) = db
        .update_player_data(notify.player_uid as i32, notify.player_data.unwrap())
        .await
    {
        error!("update_player_data failed: {err}");
    }
}

async fn fetch_player_data(
    db: &DbConnection,
    uid: i32,
) -> Result<PlayerData, BinaryDataFetchError> {
    Ok(PlayerData {
        basic: Some(db.fetch_player_basic_module_data(uid).await?),
        avatar: Some(db.fetch_model_data(uid).await?),
        item: Some(db.fetch_model_data(uid).await?),
        // TODO: store each quest collection separately
        quest: Some(db.fetch_model_data(uid).await?),
        archive: Some(db.fetch_model_data(uid).await?),
        hollow: Some(db.fetch_model_data(uid).await?),
        abyss: Some(db.fetch_model_data(uid).await?),
        buddy: Some(db.fetch_model_data(uid).await?),
        misc: Some(db.fetch_model_data(uid).await?),
        main_city: Some(db.fetch_model_data(uid).await?),
        scene: Some(db.fetch_model_data(uid).await?),
        gacha: Some(db.fetch_model_data(uid).await?),
        map: Some(db.fetch_model_data(uid).await?),
    })
}
