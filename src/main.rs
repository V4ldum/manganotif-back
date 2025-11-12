use anyhow::Result;
use manganotif_api::{AppState, Database};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    let api_secret = dotenvy::var("API_SECRET")?;

    // Setup the database connection
    let options = SqliteConnectOptions::from_str(&database_url)?
        .journal_mode(if cfg!(debug_assertions) {
            SqliteJournalMode::Delete
        } else {
            // https://x.com/levelsio/status/1867660294358806858
            //SqliteJournalMode::Wal
            SqliteJournalMode::Delete
        })
        .create_if_missing(true);
    let database = SqlitePool::connect_with(options).await?;

    // Automatically migrate the database
    sqlx::migrate!().run(&database).await?;

    // Launch the API
    manganotif_api::run(AppState {
        database: Database::new(database),
        api_secret: api_secret,
    })
    .await;

    Ok(())
}
