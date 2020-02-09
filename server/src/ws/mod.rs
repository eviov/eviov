use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net;

use crate::Plugin;

mod config;

pub async fn start<X: Plugin>(plugin: Arc<X>) -> io::Result<()> {
    let mut server = net::TcpListener::bind(("0.0.0.0", 15678)).await?;

    loop {
        let (stream, addr) = server.accept().await?;
        let _ = tokio::spawn(entry(Arc::clone(&plugin), stream, addr)); // no need to collect result
    }
}

async fn entry<X: Plugin>(plugin: Arc<X>, stream: net::TcpStream, addr: SocketAddr) {
    async fn inner<X: Plugin>(plugin: Arc<X>, stream: net::TcpStream, addr: SocketAddr) -> Result<(), String> {
        let wss = tokio_tungstenite::accept_async(stream).await.map_err(|err| err.to_string())?;
        unimplemented!()
    }

    if let Err(err) =  inner(plugin, stream, addr).await {
        log::error!("Error handling connection from {}: {}", addr, err);
    }
}
