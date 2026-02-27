use crate::models::client::Client;
use sqlx::{Pool, Row, Sqlite};

fn row_to_client(r: &sqlx::sqlite::SqliteRow) -> Client {
    Client {
        id: r.get("id"),
        name: r.get("name"),
        public_key: r.get("public_key"),
        preshared_key: r.get("preshared_key"),
        ipv4: r.get("ipv4"),
        ipv6: r.get("ipv6"),
        enabled: r.get("enabled"),
        created_at: r.get("created_at"),
        expires_at: r.get("expires_at"),
        download_url: r.get("download_url"),
        one_time_link: r.get("one_time_link"),
    }
}

const SELECT_ALL: &str = "SELECT id, name, public_key, preshared_key, ipv4, ipv6, enabled, created_at, expires_at, download_url, one_time_link FROM clients";

pub async fn list(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<Client>> {
    let rows = sqlx::query(&format!("{SELECT_ALL} ORDER BY created_at"))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(row_to_client).collect())
}

pub async fn list_enabled(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<Client>> {
    let rows = sqlx::query(&format!("{SELECT_ALL} WHERE enabled = 1"))
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(row_to_client).collect())
}

pub async fn get(pool: &Pool<Sqlite>, id: &str) -> anyhow::Result<Option<Client>> {
    let row = sqlx::query(&format!("{SELECT_ALL} WHERE id = ?"))
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row.as_ref().map(row_to_client))
}

pub async fn create(pool: &Pool<Sqlite>, client: &Client) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO clients (id, name, public_key, preshared_key, ipv4, ipv6, enabled, created_at, expires_at, download_url, one_time_link) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&client.id)
    .bind(&client.name)
    .bind(&client.public_key)
    .bind(&client.preshared_key)
    .bind(&client.ipv4)
    .bind(&client.ipv6)
    .bind(client.enabled)
    .bind(&client.created_at)
    .bind(&client.expires_at)
    .bind(&client.download_url)
    .bind(&client.one_time_link)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update(
    pool: &Pool<Sqlite>,
    id: &str,
    name: &str,
    enabled: bool,
    expires_at: Option<&str>,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE clients SET name = ?, enabled = ?, expires_at = ? WHERE id = ?")
        .bind(name)
        .bind(enabled as i64)
        .bind(expires_at)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_enabled(pool: &Pool<Sqlite>, id: &str, enabled: bool) -> anyhow::Result<()> {
    sqlx::query("UPDATE clients SET enabled = ? WHERE id = ?")
        .bind(enabled as i64)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM clients WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_used_ips(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<String>> {
    let rows = sqlx::query("SELECT ipv4 FROM clients")
        .fetch_all(pool)
        .await?;
    Ok(rows.iter().map(|r| r.get::<String, _>("ipv4")).collect())
}
