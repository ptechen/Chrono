use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use async_trait::async_trait;
use error::result::{AppResult as Result, AppResult};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, FromRow};

const FIELDS: &'static str = "id,data_type,data,is_sender,status,is_readed,is_deleted";

const CHAT_INFO_TABLE: &'static str = "create table if not exists $1
(
    id         integer default 0  not null
        constraint friends_pk
            primary key autoincrement,
    data_type     integer    default 0 not null,
    data   TEXT    default '' not null,
    is_sender integer default 0 not null,
    status integer default 0 not null,
    is_readed integer default 0 not null,
    is_deleted integer default 0  not null
);";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct ChatInfo {
    pub id: i64,
    pub data_type: u8,
    pub data: String,
    pub is_sender: u8,
    pub status: u8,
    pub is_readed: u8,
    pub is_deleted: u8,
}

#[async_trait]
impl CreateTable for ChatInfo {}

impl ChatInfo {
    pub async fn insert(&self, receiver: &str) -> Result<i64> {
        let table_name = Self::gen_chat_info_table_name(receiver).await;
        Self::create_table_by_table_name(CHAT_INFO_TABLE, &table_name).await?;
        let sql = format!(
            "INSERT INTO {} ({}) VALUES(?,?,?,?,?,?,?)",
            table_name, FIELDS
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.data_type)
            .bind(&self.data)
            .bind(&self.is_sender)
            .bind(&self.status)
            .bind(&self.is_readed)
            .bind(&self.is_deleted)
            .execute(&mut *pool)
            .await?
            .last_insert_rowid();
        Ok(data)
    }

    pub async fn select_all_by_page(
        receiver: &str,
        page_no: u64,
        page_size: u64,
    ) -> Result<Vec<Self>> {
        let table_name = Self::gen_chat_info_table_name(receiver).await;
        let sql = format!(
            "SELECT {} FROM {} order by id desc limit {},{}",
            FIELDS,
            table_name,
            (page_no - 1) * page_size,
            page_no * page_size
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .fetch_all(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn update_status_by_id(id: i64, status: u8, receiver: &str) -> AppResult<u64> {
        let table_name = Self::gen_chat_info_table_name(receiver).await;
        let sql = format!("UPDATE {} SET status = ? WHERE id = ?", table_name);
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
