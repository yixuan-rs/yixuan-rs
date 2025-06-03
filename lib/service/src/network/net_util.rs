use std::net::SocketAddr;

use tokio::net::TcpListener;

pub fn tcp_bind_sync(addr: SocketAddr) -> std::io::Result<TcpListener> {
    let listener = std::net::TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;

    Ok(TcpListener::from_std(listener).unwrap())
}
