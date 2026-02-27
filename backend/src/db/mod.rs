use std::sync::Arc;
use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};
use tracing::info;

pub type Db = Arc<Pool<Sqlite>>;

pub mod users;
pub mod clients;
pub mod interfaces;
pub mod settings;

pub async fn init_db(db_path: &str) -> anyhow::Result<Db> {
    let url = format!("sqlite://{db_path}");

    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        info!("Creating database at {db_path}");
        Sqlite::create_database(&url).await?;
    }

    let pool = SqlitePool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database migrations applied");

    Ok(Arc::new(pool))
}
