use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
// use sqlx::FromRow;
use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use error::result::{AppResult as Result, AppResult};

const TABLE_NAME: &'static str = "apply_add_friend_record";

const FIELDS: &'static str =
    "id,peer_id,avatar,nickname,comment,is_sender,status,is_readed,is_deleted";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct ApplyAddFriendRecord {
    pub id: i64,
    pub peer_id: String,
    pub avatar: String,
    pub nickname: String,
    pub comment: String,
    pub is_sender: u8,
    /// 1: 同意 2: 拒绝
    pub status: u8,
    pub is_readed: u8,
    pub is_deleted: u8,
}

const FRIENDS_TABLE: &'static str = "create table if not exists apply_add_friend_record
(
    id         integer default 0  not null
        constraint apply_add_friend_record_pk
            primary key,
    peer_id     TEXT    default '' not null,
    avatar     TEXT    default '' not null,
    nickname   TEXT    default '' not null,
    comment    Text    default '' not null,
    is_sender integer default 0 not null,
    status integer default 0 not null,
    is_readed integer default 0 not null,
    is_deleted integer default 0  not null
);";

#[async_trait]
impl CreateTable for ApplyAddFriendRecord {}

impl ApplyAddFriendRecord {
    pub async fn insert(&self) -> Result<i64> {
        Self::create_table(FRIENDS_TABLE).await?;
        let sql = format!(
            "INSERT INTO {} ({}) VALUES(?,?,?,?,?,?,?,?,?)",
            TABLE_NAME, FIELDS
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.peer_id)
            .bind(&self.avatar)
            .bind(&self.nickname)
            .bind(&self.comment)
            .bind(&self.is_sender)
            .bind(&self.status)
            .bind(&self.is_readed)
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

    pub async fn select_optional_by_id(id: i64) -> AppResult<Option<Self>> {
        let sql = format!("SELECT {} FROM {} WHERE id = ?", FIELDS, TABLE_NAME,);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .bind(id)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn update_status_by_id(id: i64, status: u8) -> AppResult<u64> {
        let sql = format!("UPDATE {} SET status = ? WHERE id = ?", TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(status)
            .bind(id)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }
}
