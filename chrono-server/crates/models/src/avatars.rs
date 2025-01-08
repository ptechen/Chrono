use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use async_trait::async_trait;
use error::result::AppResult as Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

const FIELDS: &'static str = "id,avatar";

const TABLE_NAME: &'static str = "avatars";

const AVATAR_TABLE: &'static str = "create table if not exists avatars
(
    id     varchar(16) default '' not null
        constraint avatars_pk
            primary key,
    avatar blob                   not null
);";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Avatars {
    pub id: String,
    pub avatar: Vec<u8>,
}

#[async_trait]
impl CreateTable for Avatars {}

impl Avatars {
    pub async fn insert(&self) -> Result<u64> {
        Self::create_table(AVATAR_TABLE).await?;
        let sql = format!("INSERT INTO {} ({}) VALUES(?,?)", TABLE_NAME, FIELDS);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.avatar)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }

    pub async fn update_by_id(&self) -> Result<u64> {
        let sql = format!("UPDATE {} SET avatar = ? WHERE id = ?", TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.avatar)
            .bind(&self.id)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }

    pub async fn update_insert(&self) -> Result<u64> {
        if let Ok(Some(_)) = Self::select_optional_by_id(&self.id).await {
            self.update_by_id().await
        } else {
            self.insert().await
        }
    }

    pub async fn select_optional_by_id(id: &str) -> Result<Option<Self>> {
        let sql = format!("SELECT {} FROM {} WHERE id = ?", FIELDS, TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .bind(&id)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }
}
