use crate::models::interface::Interface;
use sqlx::{Pool, Row, Sqlite};

pub async fn get(pool: &Pool<Sqlite>) -> anyhow::Result<Option<Interface>> {
    let row = sqlx::query(
        "SELECT id, name, private_key, public_key, listen_port, ipv4_cidr, ipv6_cidr FROM interfaces LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Interface {
        id: r.get("id"),
        name: r.get("name"),
        private_key: r.get("private_key"),
        public_key: r.get("public_key"),
        listen_port: r.get("listen_port"),
        ipv4_cidr: r.get("ipv4_cidr"),
        ipv6_cidr: r.get("ipv6_cidr"),
    }))
}

pub async fn upsert(pool: &Pool<Sqlite>, iface: &Interface) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO interfaces (id, name, private_key, public_key, listen_port, ipv4_cidr, ipv6_cidr) VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET name=excluded.name, private_key=excluded.private_key, public_key=excluded.public_key, listen_port=excluded.listen_port, ipv4_cidr=excluded.ipv4_cidr, ipv6_cidr=excluded.ipv6_cidr"
    )
    .bind(&iface.id)
    .bind(&iface.name)
    .bind(&iface.private_key)
    .bind(&iface.public_key)
    .bind(iface.listen_port)
    .bind(&iface.ipv4_cidr)
    .bind(&iface.ipv6_cidr)
    .execute(pool)
    .await?;
    Ok(())
}
