use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{Database, database::Manga, utils::APIError};

#[derive(Serialize)]
pub(crate) struct MangaDto {
    pub id: i64,
    pub title: String,
    pub special_edition: Option<String>,
    pub volume: Option<i64>,
    pub release_date: String,
    pub md_id: String,
    pub cover_url: Option<String>,
}

impl From<Manga> for MangaDto {
    fn from(value: Manga) -> Self {
        MangaDto {
            id: value.id,
            title: value.title,
            special_edition: value.special_edition,
            volume: value.volume,
            release_date: value.release_date,
            md_id: value.md_id,
            cover_url: value.cover_url,
        }
    }
}

#[derive(Serialize)]
struct MangasDto {
    mangas: Vec<MangaDto>,
}
pub(super) async fn get_all_mangas(State(database): State<Database>) -> Response {
    let Ok(mangas) = database.get_all_mangas().await else {
        return APIError::database_error().into_response();
    };

    Json(MangasDto {
        mangas: mangas.into_iter().map(|item| item.into()).collect(),
    })
    .into_response()
}
