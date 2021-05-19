mod pb;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
pub use pb::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8210";
    let server = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    loop {
        let (stream, _) = server.accept().await.unwrap();
        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, Request, Response, _>::from(stream).for_async();
            let (mut r, mut w) = stream.tcp_split();
            while let Some(Ok(msg)) = r.next().await {
                println!("{:?}", msg);
                let res = Response { data: msg.data };
                w.send(res).await.unwrap();
            }
        });
    }
}
