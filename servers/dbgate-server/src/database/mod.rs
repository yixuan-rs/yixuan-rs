use common::time_util;
use player_util::ModelData;
use sqlx::prelude::FromRow;
use yixuan_proto::{
    Message,
    server_only::{BasicData, PlayerData},
};
use yixuan_service::ServiceModule;

use crate::config::{ConnectionString, DbType};

mod player_util;

#[expect(dead_code)]
pub struct DbConnection(sqlx::AnyPool, DbType);

impl ServiceModule for DbConnection {
    fn run(
        self: std::sync::Arc<Self>,
        _service: std::sync::Arc<yixuan_service::ServiceContext>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(FromRow)]
struct BasicDataRow {
    #[expect(dead_code)]
    uid: i32,
    nick_name: String,
    name_change_times: i32,
    level: i32,
    exp: i32,
    create_time: i64,
    avatar_id: i32,
    control_avatar_id: i32,
    control_guise_avatar_id: i32,
    portrait_id: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum BinaryDataFetchError {
    #[error("SQL query failed: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("failed to decode data blob: {0}")]
    Decode(#[from] yixuan_proto::DecodeError),
}

impl DbConnection {
    pub async fn connect(connection_string: &ConnectionString) -> sqlx::Result<Self> {
        sqlx::any::install_default_drivers();
        let pool = sqlx::AnyPool::connect(&connection_string.to_string()).await?;

        match connection_string.db_type {
            DbType::Postgres => sqlx::migrate!("./migrations/postgres").run(&pool).await?,
            DbType::Mysql => sqlx::migrate!("./migrations/mysql").run(&pool).await?,
            DbType::Sqlite => sqlx::migrate!("./migrations/sqlite").run(&pool).await?,
        }

        Ok(Self(pool, connection_string.db_type))
    }

    pub async fn fetch_uid_for_account(&self, account_uid: &str) -> sqlx::Result<i32> {
        if let Some(uid) =
            sqlx::query_scalar("SELECT uid from t_account_uid where account_uid = ($1)")
                .bind(account_uid)
                .fetch_optional(&self.0)
                .await?
        {
            Ok(uid)
        } else {
            sqlx::query_scalar("INSERT INTO t_account_uid (account_uid) VALUES ($1) RETURNING uid")
                .bind(account_uid)
                .fetch_one(&self.0)
                .await
        }
    }

    pub async fn fetch_player_basic_module_data(&self, uid: i32) -> sqlx::Result<BasicData> {
        let row = if let Some(row) =
            sqlx::query_as::<_, BasicDataRow>("SELECT * FROM t_basic_data WHERE uid = ($1)")
                .bind(uid)
                .fetch_optional(&self.0)
                .await?
        {
            row
        } else {
            sqlx::query_as::<_, BasicDataRow>("INSERT INTO t_basic_data VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
                .bind(uid)
                .bind(uid.to_string())
                .bind(0)
                .bind(1)
                .bind(0)
                .bind(time_util::unix_timestamp_seconds())
                .bind(0)
                .bind(0)
                .bind(0)
                .bind(0)
                .fetch_one(&self.0).await?
        };

        Ok(BasicData {
            nick_name: row.nick_name,
            create_time: row.create_time,
            name_change_times: row.name_change_times as u32,
            level: row.level as u32,
            exp: row.exp as u32,
            avatar_id: row.avatar_id as u32,
            control_avatar_id: row.control_avatar_id as u32,
            control_guise_avatar_id: row.control_guise_avatar_id as u32,
            portrait_id: row.portrait_id as u32,
        })
    }

    pub async fn update_player_data(
        &self,
        uid: i32,
        player_data: PlayerData,
    ) -> Result<(), sqlx::Error> {
        // Update each module data, if it was encoded in update
        if let Some(basic_data) = player_data.basic {
            self.update_basic_model_data(uid, basic_data).await?;
        }

        macro_rules! update_models {
            ($player_data:expr, $($name:ident),*) => {
                $(if let Some($name) = $player_data.$name {
                    self.update_model_data(uid as i32, $name).await?;
                })*
            };
        }

        update_models!(
            player_data,
            avatar,
            item,
            quest,
            archive,
            hollow,
            abyss,
            buddy,
            misc,
            main_city,
            scene,
            gacha,
            map
        );

        Ok(())
    }

    async fn update_basic_model_data(&self, uid: i32, data: BasicData) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE t_basic_data SET nick_name = ($2), name_change_times = ($3), level = ($4), exp = ($5), create_time = ($6), avatar_id = ($7), control_avatar_id = ($8), portrait_id = ($9), control_guise_avatar_id = ($10) WHERE uid = ($1)")
                .bind(uid)
                .bind(data.nick_name)
                .bind(data.name_change_times as i32)
                .bind(data.level as i32)
                .bind(data.exp as i32)
                .bind(data.create_time as i32)
                .bind(data.avatar_id as i32)
                .bind(data.control_avatar_id as i32)
                .bind(data.portrait_id as i32)
                .bind(data.control_guise_avatar_id as i32)
                .execute(&self.0).await?;

        Ok(())
    }

    pub async fn fetch_model_data<Model: ModelData + Message + Default>(
        &self,
        uid: i32,
    ) -> Result<Model, BinaryDataFetchError> {
        if let Some(bytes) = sqlx::query_scalar::<_, Vec<u8>>(&format!(
            "SELECT data FROM {} WHERE uid = ($1)",
            Model::TABLE
        ))
        .bind(uid)
        .fetch_optional(&self.0)
        .await?
        {
            Ok(Model::decode(bytes.as_ref())?)
        } else {
            let data = Model::create_default(uid);
            sqlx::query(&format!("INSERT INTO {} VALUES ($1, $2)", Model::TABLE))
                .bind(uid)
                .bind(data.encode_to_vec())
                .execute(&self.0)
                .await?;

            Ok(data)
        }
    }

    pub async fn update_model_data<Data: ModelData + Message>(
        &self,
        uid: i32,
        data: Data,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "UPDATE {} SET data = ($1) WHERE uid = ($2)",
            Data::TABLE
        ))
        .bind(data.encode_to_vec())
        .bind(uid)
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
