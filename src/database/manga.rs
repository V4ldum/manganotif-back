use crate::{Database, database::Manga};
use anyhow::Result;

impl Database {
    pub(crate) async fn get_all_mangas(&self) -> Result<Vec<Manga>> {
        let result = sqlx::query_as!(Manga, "SELECT * FROM manga_release")
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }
}
