use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub id: String,
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub listen_port: i64,
    pub ipv4_cidr: String,
    pub ipv6_cidr: Option<String>,
}
