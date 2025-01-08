use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::fs::read;
// use sqlx::FromRow;
use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use error::result::{AppResult as Result, AppResult};

const TABLE_NAME: &'static str = "friends";

const FIELDS: &'static str = "id,pub_key,peer_id,avatar,nickname,is_group,is_deleted";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Friends {
    pub id: i64,
    pub pub_key: String,
    pub peer_id: String,
    pub avatar: String,
    pub nickname: String,
    pub is_group: u8,
    pub is_deleted: u8,
}

const FRIENDS_TABLE: &'static str = "create table if not exists friends
(
    id         integer default 0  not null
        constraint friends_pk
            primary key,
    pub_key    Text    default '' not null,
    peer_id     TEXT    default '' not null,
    avatar     TEXT    default '' not null,
    nickname   TEXT    default '' not null,
    is_group integer default 0 not null,
    is_deleted integer default 0  not null
);";

#[async_trait]
impl CreateTable for Friends {}

impl Friends {
    pub async fn insert(&self) -> Result<i64> {
        Self::create_table(FRIENDS_TABLE).await?;
        let sql = format!(
            "INSERT INTO {} ({}) VALUES(?,?,?,?,?,?,?)",
            TABLE_NAME, FIELDS
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.pub_key)
            .bind(&self.peer_id)
            .bind(&self.avatar)
            .bind(&self.nickname)
            .bind(&self.is_group)
            .bind(&self.is_deleted)
            .execute(&mut *pool)
            .await?
            .last_insert_rowid();
        Ok(data)
    }

    pub async fn select_all_by_page(page_no: i64, page_size: i64) -> AppResult<Vec<Self>> {
        let sql = format!(
            "SELECT {} FROM {} limit {}, {}",
            FIELDS,
            TABLE_NAME,
            (page_no - 1) * page_size,
            page_no * page_size
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .fetch_all(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn select_optional_by_peer_id(peer_id: &str) -> AppResult<Option<Self>> {
        let sql = format!("SELECT {} FROM {} WHERE peer_id = ?", FIELDS, TABLE_NAME,);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .bind(peer_id)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn select_nickname_avatar(
        peer_id: &str,
    ) -> AppResult<Option<(String, String, Vec<u8>)>> {
        if let Ok(Some(data)) = Self::select_optional_by_peer_id(peer_id).await {
            let avatar = read(&data.avatar).await?;
            Ok(Some((
                data.nickname.to_string(),
                data.avatar.to_string(),
                avatar,
            )))
        } else {
            Ok(None)
        }
    }
}
