use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use async_trait::async_trait;
use error::result::AppResult as Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

const TABLE_NAME: &'static str = "owner_info";
const FIELDS: &'static str = "email,password,phrase,is_deleted,updated_at,created_at";

const OWNER_INFO_TABLE: &'static str = "create table if not exists owner_info
(
    email    TEXT    default '' not null,
    password TEXT    default '' not null,
    phrase   TEXT    default '' not null,
    is_deleted integer default 0  not null,
    updated_at integer default 0  not null,
    created_at integer default 0  not null
);";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct OwnerInfo {
    pub email: String,
    pub password: String,
    pub phrase: String,
    pub is_deleted: u8,
    pub updated_at: i64,
    pub created_at: i64,
}

#[async_trait]
impl CreateTable for OwnerInfo {}

impl OwnerInfo {
    pub async fn insert(&self) -> Result<u64> {
        Self::create_table(OWNER_INFO_TABLE).await?;
        let sql = format!(
            "INSERT INTO {} ({}) VALUES(?,?,?,?,?,?)",
            TABLE_NAME, FIELDS
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.email)
            .bind(&self.password)
            .bind(&self.phrase)
            .bind(&self.is_deleted)
            .bind(&self.updated_at)
            .bind(&self.created_at)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }

    pub async fn select_optional_by_email_password(
        email: &str,
        password: &str,
    ) -> Result<Option<Self>> {
        let sql = format!(
            "SELECT {} FROM {} WHERE email = ? AND password = ?",
            FIELDS, TABLE_NAME
        );
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .bind(email)
            .bind(password)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn select_optional() -> Result<Option<Self>> {
        let sql = format!("SELECT {} FROM {} limit 0,1", FIELDS, TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn update_password_by_email(email: &str, password: &str) -> Result<u64> {
        let sql = format!("UPDATE {} SET password = ? WHERE email = ?", TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(password)
            .bind(email)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }
}
