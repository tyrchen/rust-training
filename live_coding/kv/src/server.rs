mod noise_codec;
mod pb;

use anyhow::Result;
use noise_codec::{NoiseCodec, NoiseStream, NOISE_PARAMS};
use std::{convert::TryInto, sync::Arc};

use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use pb::{request::*, *};
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;

    info!("Listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New client: {:?} accepted", addr);

        let shared = state.clone();

        tokio::spawn(async move {
            let mut stream = NoiseCodec::builder(NOISE_PARAMS, false).new_framed(stream)?;
            stream.handshake().await?;

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => match shared.store.get(&key) {
                        Some(v) => Response::new(key, v.value().to_vec()),
                        None => Response::not_found(key),
                    },
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    None => unimplemented!(),
                };
                stream.send(response.into()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }
}
