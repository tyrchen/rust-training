use anyhow::{anyhow, Result};
use s2n_quic::{client::Connect, Client};
use std::net::SocketAddr;
use tokio::io;

const CERT_PEM: &str = include_str!("../fixtures/cert.pem");

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_tls(CERT_PEM)?
        .with_io("0.0.0.0:0")?
        .start()
        .map_err(|e| anyhow!("Failed to start client. Error: {e}"))?;

    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut conn = client.connect(connect).await?;

    conn.keep_alive(true)?;

    // open a new bidirectional stream
    let stream = conn.open_bidirectional_stream().await?;
    let (mut rx, mut tx) = stream.split();

    println!("Connected to {}", conn.remote_addr()?);
    // spawn tokio task to copy server data to stdout
    tokio::spawn(async move {
        let mut stdout = io::stdout();
        if let Err(e) = io::copy(&mut rx, &mut stdout).await {
            println!("Failed to copy data from server. Error: {e}");
        }
    });

    // copy stdin to server
    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut tx).await?;

    Ok(())
}
