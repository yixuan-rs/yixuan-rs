use std::sync::Arc;

use futures::try_join; // Add this new import
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, warn};
use yixuan_proto::{
    server_only::{
        PlayerData, PlayerDataChangedNotify, PlayerGetDataReq, PlayerGetDataRsp, // Existing
        AbyssData, ArchiveData, AvatarData, BasicData, BigSceneData, BuddyData, // Added
        GachaData, HollowData, ItemData, MainCityData, MapData, MiscData,       // Added
        QuestData, SceneData                                                    // Added
    },
    NetCmd, PlayerGetTokenCsReq, PlayerGetTokenScRsp,
    head::PacketHead,
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
    db: &DbConnection, // This is crate::database::DbConnection
    uid: i32,
) -> Result<PlayerData, BinaryDataFetchError> { // BinaryDataFetchError from crate::database
    let basic_fut = async {
        db.fetch_player_basic_module_data(uid).await.map_err(BinaryDataFetchError::from)
    };
    let avatar_fut = db.fetch_model_data::<AvatarData>(uid);
    let item_fut = db.fetch_model_data::<ItemData>(uid);
    let quest_fut = db.fetch_model_data::<QuestData>(uid);
    let archive_fut = db.fetch_model_data::<ArchiveData>(uid);
    let hollow_fut = db.fetch_model_data::<HollowData>(uid);
    let abyss_fut = db.fetch_model_data::<AbyssData>(uid);
    let buddy_fut = db.fetch_model_data::<BuddyData>(uid);
    let misc_fut = db.fetch_model_data::<MiscData>(uid);
    let main_city_fut = db.fetch_model_data::<MainCityData>(uid);
    let scene_fut = db.fetch_model_data::<SceneData>(uid);
    let gacha_fut = db.fetch_model_data::<GachaData>(uid);
    let map_fut = db.fetch_model_data::<MapData>(uid);
    let big_scene_fut = db.fetch_model_data::<BigSceneData>(uid);

    let (
        basic_data,
        avatar_data,
        item_data,
        quest_data,
        archive_data,
        hollow_data,
        abyss_data,
        buddy_data,
        misc_data,
        main_city_data,
        scene_data,
        gacha_data,
        map_data,
        big_scene_data,
    ) = try_join!(
        basic_fut,
        avatar_fut,
        item_fut,
        quest_fut,
        archive_fut,
        hollow_fut,
        abyss_fut,
        buddy_fut,
        misc_fut,
        main_city_fut,
        scene_fut,
        gacha_fut,
        map_fut,
        big_scene_fut
    )?;

    Ok(PlayerData {
        basic: Some(basic_data),
        avatar: Some(avatar_data),
        item: Some(item_data),
        quest: Some(quest_data),
        archive: Some(archive_data),
        hollow: Some(hollow_data),
        abyss: Some(abyss_data),
        buddy: Some(buddy_data),
        misc: Some(misc_data),
        main_city: Some(main_city_data),
        scene: Some(scene_data),
        gacha: Some(gacha_data),
        map: Some(map_data),
        big_scene: Some(big_scene_data),
    })
}
