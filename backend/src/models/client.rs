use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub preshared_key: String,
    pub ipv4: String,
    pub ipv6: Option<String>,
    pub enabled: i64,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub download_url: Option<String>,
    pub one_time_link: Option<String>,
}
