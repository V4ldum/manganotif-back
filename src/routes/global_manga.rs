use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{Database, database::GlobalManga, utils::APIError};

#[derive(Serialize)]
pub(crate) struct GlobalMangaDto {
    pub id: i64,
    pub title: String,
    pub special_edition: Option<String>,
    pub volume: Option<i64>,
    pub release_date: String,
    pub cover_url: Option<String>,
}

impl From<GlobalManga> for GlobalMangaDto {
    fn from(value: GlobalManga) -> Self {
        GlobalMangaDto {
            id: value.id,
            title: value.title,
            special_edition: value.special_edition,
            volume: value.volume,
            release_date: value.release_date,
            cover_url: value.cover_url,
        }
    }
}

#[derive(Serialize)]
struct GlobalMangasDto {
    mangas: Vec<GlobalMangaDto>,
}

pub(super) async fn get_all_global_mangas(State(database): State<Database>) -> Response {
    let Ok(global_mangas) = database.get_all_global_mangas().await else {
        return APIError::database_error().into_response();
    };

    Json(GlobalMangasDto {
        mangas: global_mangas.into_iter().map(|item| item.into()).collect(),
    })
    .into_response()
}
