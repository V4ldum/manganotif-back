pub(crate) struct Manga {
    pub id: i64,
    pub title: String,
    pub special_edition: Option<String>,
    pub volume: Option<i64>,
    pub release_date: String,
    pub md_id: String,
    pub cover_url: Option<String>,
}
