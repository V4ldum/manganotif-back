use crate::{Database, database::GlobalManga};
use anyhow::Result;

impl Database {
    pub(crate) async fn get_all_global_mangas(&self) -> Result<Vec<GlobalManga>> {
        let result = sqlx::query_as!(GlobalManga, "SELECT * FROM global_manga")
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }
}
