mod pb;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
pub use pb::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:8210").await.unwrap();
    let mut stream = AsyncProstStream::<_, Response, Request, _>::from(stream).for_async();
    let req = Request {
        id: "tyr".to_owned(),
        data: "hello world".to_owned(),
    };

    let (mut r, mut w) = stream.tcp_split();
    w.send(req).await.unwrap();
    if let Ok(msg) = r.next().await.unwrap() {
        println!("Got response: {:?}", msg);
    }
}
