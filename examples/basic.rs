use std::net::Ipv4Addr;

use ipnetwork::Ipv4Network;
use lib::{get_links, add_address, add_route_v4};
use anyhow::Result;
use rtnetlink::new_connection;

#[tokio::main]
async fn main() -> Result<()> {
    let (con, handle, _) = new_connection().expect("no new connection");
    tokio::spawn(con);
    
    let links = get_links(&handle).await?;

    for (idx, name) in links {
        println!("link: {}:{}",idx, name);
    }

    let addr: Ipv4Addr = if let Ok(addr) = "192.168.0.60".parse() {
        addr
    } else {
        eprintln!("invalid ip address");
        std::process::exit(1);
    };

    let mask: Ipv4Addr = if let Ok(mask) = "255.255.255.0".parse() {
        mask
    } else {
        eprintln!("invalid subnet mask");
        std::process::exit(1);
    };

    let addr_ip = std::net::IpAddr::V4(addr);
    let mask_ip = std::net::IpAddr::V4(mask);

    let dest = if let Ok(dest) = "0.0.0.0".parse() {
        dest 
    } else {
        eprintln!("invalid route");
        std::process::exit(1);
    };

    let gw = if let Ok(gw) = "192.168.0.1".parse() {
        gw
    } else {
        eprintln!("invalid gateway");
        std::process::exit(1);
    };

    add_address(&handle, "ens4", addr_ip, mask_ip).await?;
    add_route_v4(&handle, dest, 0, gw).await?;

    Ok(())
}
