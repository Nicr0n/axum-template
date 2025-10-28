use std::{io, net::SocketAddr};

use infra::server::config::ServerConfig;
use tokio::net::lookup_host;

pub async fn get_socket_address(server_config: ServerConfig) -> Result<SocketAddr, io::Error> {
    // 1. 构造 (host, port) 元组
    let host_port = (
        server_config.basic_server_config.addr.as_str(),
        server_config.basic_server_config.port,
    );
    let mut addrs = lookup_host(host_port).await?;

    match addrs.next() {
        Some(addr) => Ok(addr),
        None => Err(io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "DNS lookup returned no addresses",
        )),
    }
}
