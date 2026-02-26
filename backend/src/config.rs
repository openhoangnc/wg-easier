use anyhow::{bail, Context};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct AppConfig {
    // Network
    pub wg_host: String,
    pub wg_port: u16,
    pub wg_mtu: Option<u16>,
    pub wg_default_address: String,
    pub wg_default_dns: String,
    pub wg_allowed_ips: String,
    pub wg_pre_up: Option<String>,
    pub wg_post_up: Option<String>,
    pub wg_pre_down: Option<String>,
    pub wg_post_down: Option<String>,
    // UI/Auth
    pub port: u16,
    pub insecure: bool,
    pub password_hash: Option<String>,
    // Paths
    pub db_path: String,
    pub static_path: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let wg_host = std::env::var("WG_HOST")
            .context("WG_HOST environment variable is required")?;

        if wg_host.is_empty() {
            bail!("WG_HOST must not be empty");
        }

        let wg_port: u16 = std::env::var("WG_PORT")
            .unwrap_or_else(|_| "51820".to_string())
            .parse()
            .context("WG_PORT must be a valid port number")?;

        let wg_mtu: Option<u16> = std::env::var("WG_MTU")
            .ok()
            .map(|v| v.parse().context("WG_MTU must be a valid number"))
            .transpose()?;

        let wg_default_address = std::env::var("WG_DEFAULT_ADDRESS")
            .unwrap_or_else(|_| "10.8.0.x".to_string());

        let wg_default_dns = std::env::var("WG_DEFAULT_DNS")
            .unwrap_or_else(|_| "1.1.1.1".to_string());

        let wg_allowed_ips = std::env::var("WG_ALLOWED_IPS")
            .unwrap_or_else(|_| "0.0.0.0/0".to_string());

        // Shell hooks are not supported in scratch image — log warning and ignore
        let wg_pre_up = std::env::var("WG_PRE_UP").ok();
        let wg_post_up = std::env::var("WG_POST_UP").ok();
        let wg_pre_down = std::env::var("WG_PRE_DOWN").ok();
        let wg_post_down = std::env::var("WG_POST_DOWN").ok();

        for (name, val) in [
            ("WG_PRE_UP", &wg_pre_up),
            ("WG_POST_UP", &wg_post_up),
            ("WG_PRE_DOWN", &wg_pre_down),
            ("WG_POST_DOWN", &wg_post_down),
        ] {
            if val.is_some() {
                warn!(
                    "{} is set but shell hooks are not supported in the FROM-scratch image; ignoring",
                    name
                );
            }
        }

        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "51821".to_string())
            .parse()
            .context("PORT must be a valid port number")?;

        let insecure = std::env::var("INSECURE")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase()
            == "true";

        let password_hash = std::env::var("PASSWORD_HASH").ok();

        if !insecure && password_hash.is_none() {
            bail!("Either set PASSWORD_HASH or enable INSECURE=true");
        }

        let db_path = std::env::var("WG_DB_PATH")
            .unwrap_or_else(|_| "/etc/wireguard/wg-easy.db".to_string());

        let static_path = std::env::var("WG_STATIC_PATH")
            .unwrap_or_else(|_| "/app/static".to_string());

        Ok(Self {
            wg_host,
            wg_port,
            wg_mtu,
            wg_default_address,
            wg_default_dns,
            wg_allowed_ips,
            wg_pre_up,
            wg_post_up,
            wg_pre_down,
            wg_post_down,
            port,
            insecure,
            password_hash,
            db_path,
            static_path,
        })
    }
}
