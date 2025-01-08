use crate::create_table::CreateTable;
use crate::SQLITE_POOL;
use async_trait::async_trait;
use error::result::AppResult as Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

const TABLE_NAME: &'static str = "verification_code";
const FIELDS: &'static str = "code,created_at";

const VERIFICATION_CODE: &'static str = "create table if not exists verification_code
(
    code    TEXT    default '' not null,
    created_at integer default 0  not null
);";

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct VerificationCode {
    pub code: String,
    pub created_at: i64,
}

#[async_trait]
impl CreateTable for VerificationCode {}

impl VerificationCode {
    pub async fn insert(&self) -> Result<u64> {
        Self::create_table(VERIFICATION_CODE).await?;
        Self::delete_all().await?;
        let sql = format!("INSERT INTO {} ({}) VALUES(?,?)", TABLE_NAME, FIELDS);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query(&sql)
            .bind(&self.code)
            .bind(&self.created_at)
            .execute(&mut *pool)
            .await?
            .rows_affected();
        Ok(data)
    }

    pub async fn select_optional_by_code(code: &str) -> Result<Option<Self>> {
        let sql = format!("SELECT {} FROM {} WHERE code = ?", FIELDS, TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .bind(code)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }

    pub async fn delete_all() -> Result<Option<Self>> {
        let sql = format!("DELETE FROM {} where created_at > 0", TABLE_NAME);
        let mut pool = SQLITE_POOL.acquire().await?;
        let data = sqlx::query_as::<_, Self>(&sql)
            .fetch_optional(&mut *pool)
            .await?;
        Ok(data)
    }
}
