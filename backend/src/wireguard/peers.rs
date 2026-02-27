use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use wireguard_control::{
    Backend, Device, DeviceUpdate, InterfaceName, Key, PeerConfigBuilder,
};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStats {
    pub public_key: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub last_handshake_secs: Option<u64>,
}

fn iface_name(name: &str) -> anyhow::Result<InterfaceName> {
    InterfaceName::from_str(name).map_err(|e| anyhow!("Invalid interface name {}: {}", name, e))
}

/// Configure the WireGuard interface with private key and listen port.
pub fn configure_interface(name: &str, private_key_b64: &str, listen_port: u16) -> anyhow::Result<()> {
    let iface = iface_name(name)?;
    let key = Key::from_base64(private_key_b64)
        .map_err(|e| anyhow!("Invalid private key: {}", e))?;
    DeviceUpdate::new()
        .set_private_key(key)
        .set_listen_port(listen_port)
        .apply(&iface, Backend::Kernel)?;
    Ok(())
}

/// Add or update a peer on the WireGuard interface.
pub fn add_peer(
    name: &str,
    public_key_b64: &str,
    preshared_key_b64: &str,
    allowed_ips: &[&str],
) -> anyhow::Result<()> {
    let iface = iface_name(name)?;
    let pubkey = Key::from_base64(public_key_b64)
        .map_err(|e| anyhow!("Invalid public key: {}", e))?;
    let psk = Key::from_base64(preshared_key_b64)
        .map_err(|e| anyhow!("Invalid preshared key: {}", e))?;

    let mut peer = PeerConfigBuilder::new(&pubkey).set_preshared_key(psk);
    for ip_str in allowed_ips {
        let net: ipnet::IpNet = ip_str.parse()
            .map_err(|e| anyhow!("Invalid allowed IP {}: {}", ip_str, e))?;
        peer = peer.add_allowed_ip(net.addr(), net.prefix_len());
    }

    DeviceUpdate::new()
        .add_peer(peer)
        .apply(&iface, Backend::Kernel)?;
    Ok(())
}

/// Remove a peer from the WireGuard interface.
pub fn remove_peer(name: &str, public_key_b64: &str) -> anyhow::Result<()> {
    let iface = iface_name(name)?;
    let pubkey = Key::from_base64(public_key_b64)
        .map_err(|e| anyhow!("Invalid public key: {}", e))?;
    DeviceUpdate::new()
        .remove_peer_by_key(&pubkey)
        .apply(&iface, Backend::Kernel)?;
    Ok(())
}

/// Read stats for all peers on the WireGuard interface.
pub fn get_stats(name: &str) -> anyhow::Result<Vec<PeerStats>> {
    let iface = iface_name(name)?;
    let device = Device::get(&iface, Backend::Kernel)?;
    let stats = device
        .peers
        .into_iter()
        .map(|p| PeerStats {
            public_key: p.config.public_key.to_base64(),
            rx_bytes: p.stats.rx_bytes,
            tx_bytes: p.stats.tx_bytes,
            last_handshake_secs: p.stats.last_handshake_time
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
        })
        .collect();
    Ok(stats)
}

/// Bulk-sync all peers on startup.
pub fn sync_peers(
    name: &str,
    peers: &[(String, String, Vec<String>)], // (public_key, preshared_key, allowed_ips)
) -> anyhow::Result<()> {
    let iface = iface_name(name)?;
    let mut update = DeviceUpdate::new().replace_peers();
    for (pubkey_b64, psk_b64, allowed_ips) in peers {
        let pubkey = Key::from_base64(pubkey_b64)
            .map_err(|e| anyhow!("Invalid public key: {}", e))?;
        let psk = Key::from_base64(psk_b64)
            .map_err(|e| anyhow!("Invalid preshared key: {}", e))?;
        let mut peer = PeerConfigBuilder::new(&pubkey).set_preshared_key(psk);
        for ip_str in allowed_ips {
            let net: ipnet::IpNet = ip_str.parse()
                .map_err(|e| anyhow!("Invalid allowed IP {}: {}", ip_str, e))?;
            peer = peer.add_allowed_ip(net.addr(), net.prefix_len());
        }
        update = update.add_peer(peer);
    }
    update.apply(&iface, Backend::Kernel)?;
    Ok(())
}
