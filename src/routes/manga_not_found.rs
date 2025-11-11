use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{Database, database::MangaNotFound, utils::APIError};

#[derive(Serialize)]
pub(crate) struct MangaNotFoundDto {
    pub id: i64,
    pub title: String,
    pub special_edition: Option<String>,
    pub volume: Option<i64>,
    pub release_date: String,
    pub cover_url: Option<String>,
    pub titles: Option<String>,
}

impl From<MangaNotFound> for MangaNotFoundDto {
    fn from(value: MangaNotFound) -> Self {
        MangaNotFoundDto {
            id: value.id,
            title: value.title,
            special_edition: value.special_edition,
            volume: value.volume,
            release_date: value.release_date,
            cover_url: value.cover_url,
            titles: value.titles,
        }
    }
}

#[derive(Serialize)]
struct MangasNotFoundDto {
    mangas: Vec<MangaNotFoundDto>,
}
pub(super) async fn get_all_mangas_not_found(State(database): State<Database>) -> Response {
    let Ok(mangas_not_found) = database.get_all_mangas_not_found().await else {
        return APIError::database_error().into_response();
    };

    Json(MangasNotFoundDto {
        mangas: mangas_not_found
            .into_iter()
            .map(|item| item.into())
            .collect(),
    })
    .into_response()
}
