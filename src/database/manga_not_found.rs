use crate::{Database, database::MangaNotFound};
use anyhow::Result;

impl Database {
    pub(crate) async fn get_all_mangas_not_found(&self) -> Result<Vec<MangaNotFound>> {
        let result = sqlx::query_as!(MangaNotFound, "SELECT * FROM manga_not_found")
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }
}
