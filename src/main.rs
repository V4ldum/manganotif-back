use anyhow::{Context, Result};
use log::LevelFilter;
use manganotif_api::{AppState, Database};
use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TermLogger, TerminalMode};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;

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

// Error levels
// Trace - Only when I would be "tracing" the code and trying to find one part of a function specifically.
// Debug - Information that is diagnostically helpful to people more than just developers (IT, sysadmins, etc.).
// Info  - Generally useful information to log (service start/stop, configuration assumptions, etc).
//         Info I want to always have available but usually don't care about under normal circumstances.
//         This is my out-of-the-box config level.
// Warn  - Anything that can potentially cause application oddities, but for which I am automatically recovering.
//         (Such as switching from a primary to backup server, retrying an operation, missing secondary data, etc.)
// Error - Any error which is fatal to the operation, but not the service or application (can't open a required file,
//         missing data, etc.). These errors will force user (administrator, or direct user) intervention.
//         These are usually reserved (in my apps) for incorrect connection strings, missing services, etc.
fn setup_logging() -> Result<()> {
    let logger_config = ConfigBuilder::new()
        .set_time_level(if cfg!(debug_assertions) {
            LevelFilter::Off // Time for nothing
        } else {
            LevelFilter::Error // Time for everything
        })
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_level_padding(LevelPadding::Left)
        .add_filter_allow_str("manganotif")
        .build();

    TermLogger::init(
        if cfg!(debug_assertions) {
            LevelFilter::Trace
        } else {
            LevelFilter::Warn
        },
        logger_config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .context("Failed to register logger")?;

    Ok(())
}
