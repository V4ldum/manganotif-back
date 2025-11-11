mod global_manga;
mod manga;
mod manga_not_found;
mod structs;

pub(crate) use structs::GlobalManga;
pub(crate) use structs::Manga;
pub(crate) use structs::MangaNotFound;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database {
    db: SqlitePool,
}

impl Database {
    pub fn new(db: SqlitePool) -> Self {
        Database { db }
    }
}
