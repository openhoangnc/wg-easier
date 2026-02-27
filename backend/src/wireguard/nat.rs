use anyhow::anyhow;
use tracing::info;

const TABLE_NAME: &str = "wg_easy_nat";
const CHAIN_NAME: &str = "postrouting";

/// Set up NAT MASQUERADE using nftables for the WireGuard subnet.
///
/// # Arguments
/// * `wg_cidr` - The WireGuard subnet in CIDR notation (e.g. `"10.8.0.0/24"`)
/// * `outbound_iface` - The physical network interface used for outbound traffic (e.g. `"eth0"`)
pub fn setup_nat(wg_cidr: &str, outbound_iface: &str) -> anyhow::Result<()> {
    use rustables::{
        Batch, Chain, ChainPolicy, ChainType, Hook, HookClass, MsgType, ProtocolFamily, Rule, Table,
    };
    use rustables::expr::Masquerade;

    let mut batch = Batch::new();

    // Create table
    let table = Table::new(ProtocolFamily::Inet)
        .with_name(TABLE_NAME.to_string());
    batch.add(&table, MsgType::Add);

    // Create nat chain hooked at postrouting
    let hook = Hook::new(HookClass::PostRouting, 100);
    let chain = Chain::new(&table)
        .with_name(CHAIN_NAME.to_string())
        .with_hook(hook)
        .with_type(ChainType::Nat)
        .with_policy(ChainPolicy::Accept);
    batch.add(&chain, MsgType::Add);

    // Add masquerade rule
    let rule = Rule::new(&chain)
        .map_err(|e| anyhow!("Rule build error: {:?}", e))?
        .with_expr(Masquerade {});
    batch.add(&rule, MsgType::Add);

    batch.send()
        .map_err(|e| anyhow!("nftables batch send error: {:?}", e))?;

    info!("nftables NAT MASQUERADE configured for {} via {}", wg_cidr, outbound_iface);
    Ok(())
}

/// Remove the wg_easy_nat nftables table atomically.
pub fn teardown_nat() -> anyhow::Result<()> {
    use rustables::{Batch, MsgType, ProtocolFamily, Table};

    let mut batch = Batch::new();
    let table = Table::new(ProtocolFamily::Inet)
        .with_name(TABLE_NAME.to_string());
    batch.add(&table, MsgType::Del);
    batch.send()
        .map_err(|e| anyhow!("nftables teardown error: {:?}", e))?;

    info!("nftables NAT table {} removed", TABLE_NAME);
    Ok(())
}
