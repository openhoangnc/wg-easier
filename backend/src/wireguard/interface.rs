use anyhow::anyhow;
use futures::TryStreamExt;
use ipnet::IpNet;
use rtnetlink::Handle;
use tracing::info;

/// Create a WireGuard network interface.
pub async fn create_wireguard_link(handle: &Handle, name: &str) -> anyhow::Result<()> {
    handle
        .link()
        .add()
        .wireguard(name.to_string())
        .execute()
        .await
        .map_err(|e| anyhow!("Failed to create wireguard link {}: {}", name, e))?;
    info!("Created WireGuard interface {}", name);
    Ok(())
}

/// Bring a network interface up by index.
pub async fn set_link_up(handle: &Handle, index: u32) -> anyhow::Result<()> {
    handle
        .link()
        .set(index)
        .up()
        .execute()
        .await
        .map_err(|e| anyhow!("Failed to set link {} up: {}", index, e))?;
    Ok(())
}

/// Assign a CIDR address to a network interface.
pub async fn assign_address(handle: &Handle, index: u32, net: &IpNet) -> anyhow::Result<()> {
    let prefix_len = net.prefix_len();
    handle
        .address()
        .add(index, net.addr(), prefix_len)
        .execute()
        .await
        .map_err(|e| anyhow!("Failed to assign address: {}", e))?;
    Ok(())
}

/// Add a route for the VPN subnet.
pub async fn add_route(handle: &Handle, index: u32, network: &IpNet) -> anyhow::Result<()> {
    match network {
        IpNet::V4(net) => {
            handle
                .route()
                .add()
                .v4()
                .destination_prefix(net.network(), net.prefix_len())
                .output_interface(index)
                .execute()
                .await
                .map_err(|e| anyhow!("Failed to add IPv4 route: {}", e))?;
        }
        IpNet::V6(net) => {
            handle
                .route()
                .add()
                .v6()
                .destination_prefix(net.network(), net.prefix_len())
                .output_interface(index)
                .execute()
                .await
                .map_err(|e| anyhow!("Failed to add IPv6 route: {}", e))?;
        }
    }
    Ok(())
}

/// Delete a network interface by name.
pub async fn delete_link(handle: &Handle, name: &str) -> anyhow::Result<()> {
    let index = get_link_index(handle, name).await?;
    handle
        .link()
        .del(index)
        .execute()
        .await
        .map_err(|e| anyhow!("Failed to delete link {}: {}", name, e))?;
    info!("Deleted WireGuard interface {}", name);
    Ok(())
}

/// Resolve an interface name to its index.
pub async fn get_link_index(handle: &Handle, name: &str) -> anyhow::Result<u32> {
    let mut links = handle.link().get().match_name(name.to_string()).execute();
    if let Some(msg) = links.try_next().await
        .map_err(|e| anyhow!("rtnetlink error: {}", e))?
    {
        Ok(msg.header.index)
    } else {
        Err(anyhow!("Interface {} not found", name))
    }
}

/// Check whether a WireGuard interface already exists.
pub async fn link_exists(handle: &Handle, name: &str) -> bool {
    get_link_index(handle, name).await.is_ok()
}
