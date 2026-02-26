use sqlx::{Pool, Sqlite, Row};

#[allow(dead_code)]
pub async fn get(pool: &Pool<Sqlite>, key: &str) -> anyhow::Result<Option<String>> {
    let row = sqlx::query("SELECT value FROM config WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.get("value")))
}

pub async fn set(pool: &Pool<Sqlite>, key: &str, value: &str) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO config (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value"
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn get_all(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<(String, String)>> {
    let rows = sqlx::query("SELECT key, value FROM config")
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|r| (r.get("key"), r.get("value"))).collect())
}
