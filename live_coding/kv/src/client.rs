mod noise_codec;
mod pb;

use std::convert::TryFrom;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use noise_codec::{NoiseCodec, NoiseStream, NOISE_PARAMS};
use pb::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = "localhost:8888";
    let stream = TcpStream::connect(addr).await?;
    let mut stream = NoiseCodec::builder(NOISE_PARAMS, true).new_framed(stream)?;

    stream.handshake().await?;

    let msg = Request::new_put("hello", b"world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("hello");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        let msg = Response::try_from(buf)?;
        println!("Got msg: {:?}", msg);
    }

    Ok(())
}
