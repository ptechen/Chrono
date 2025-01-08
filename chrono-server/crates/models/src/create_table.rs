use crate::SQLITE_POOL;
use error::result::AppResult as Result;

#[async_trait::async_trait]
pub trait CreateTable {
    async fn create_table(table_sql: &str) -> Result<()> {
        let mut pool = SQLITE_POOL.acquire().await?;
        sqlx::query(table_sql).execute(&mut *pool).await?;
        Ok(())
    }

    async fn create_table_by_table_name(table_sql: &str, table_name: &str) -> Result<()> {
        let sql = table_sql.replace("$1", table_name);
        Self::create_table(&sql).await?;
        Ok(())
    }

    async fn gen_chat_info_table_name(receiver: &str) -> String {
        format!("chat_info_{}", receiver)
    }
}
