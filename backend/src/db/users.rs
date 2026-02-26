use sqlx::{Pool, Sqlite, Row};
use crate::models::user::User;

pub async fn find_by_username(pool: &Pool<Sqlite>, username: &str) -> anyhow::Result<Option<User>> {
    let row = sqlx::query(
        "SELECT id, username, password_hash, totp_secret FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.get("id"),
        username: r.get("username"),
        password_hash: r.get("password_hash"),
        totp_secret: r.get("totp_secret"),
    }))
}

pub async fn create(pool: &Pool<Sqlite>, username: &str, password_hash: &str) -> anyhow::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)"
    )
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn set_totp_secret(pool: &Pool<Sqlite>, id: i64, secret: Option<&str>) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET totp_secret = ? WHERE id = ?")
        .bind(secret)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_password(pool: &Pool<Sqlite>, id: i64, password_hash: &str) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(password_hash)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
