use std::net::{IpAddr, Ipv4Addr};

use anyhow::Result;
use futures::stream::TryStreamExt;
use ipnetwork::IpNetwork;
use rtnetlink::packet::nlas::link::Nla;

pub use rtnetlink;

pub async fn get_links(handle: &rtnetlink::Handle) -> Result<Vec<(u32, String)>> {
    let mut rc = Vec::new();

    let mut links = handle.link().get().execute();
    while let Some(msg) = links.try_next().await? {
        let mut x = msg
            .nlas
            .into_iter()
            .filter_map(|nla| {
                if let Nla::IfName(name) = nla {
                    Some((msg.header.index, name.clone()))
                } else {
                    None
                }
            })
            .collect();
        rc.append(&mut x);
    }
    Ok(rc)
}

pub async fn add_address(
    handle: &rtnetlink::Handle,
    link_name: &str,
    address: IpAddr,
    mask: IpAddr,
) -> Result<()> {
    let mut links = handle
        .link()
        .get()
        .match_name(link_name.to_string())
        .execute();

    if address.is_ipv4() != mask.is_ipv4() {
        anyhow::bail!("address and mask are different ip protocols");
    }

    if let Ok(ip) = IpNetwork::with_netmask(address, mask) {
        if let Some(link) = links.try_next().await? {
            handle.link().set(link.header.index).up().execute().await?;

            handle
                .address()
                .add(link.header.index, ip.ip(), ip.prefix())
                .execute()
                .await?;
        }
    }

    Ok(())
}

pub async fn add_route_v4(
    handle: &rtnetlink::Handle,
    dest: Ipv4Addr,
    prefix: u8,
    gw: Ipv4Addr,
) -> Result<()> {
    let route = handle.route();

    route
        .add()
        .v4()
        .destination_prefix(dest, prefix)
        .gateway(gw)
        .execute()
        .await?;
    Ok(())
}
