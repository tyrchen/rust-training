use crate::pb::{
    echo_service_client::EchoServiceClient,
    echo_service_server::{EchoService, EchoServiceServer},
    EchoRequest, EchoResponse,
};
use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct Echoer {}

#[tonic::async_trait]
impl EchoService for Echoer {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let reply = EchoResponse {
            message: req.message,
        };

        Ok(Response::new(reply))
    }
}

pub async fn start_server() -> Result<()> {
    let addr = "[::1]:5001".parse()?;
    let echoer = Echoer::default();
    println!("Echoer listening on {}", addr);

    let svc = EchoServiceServer::new(echoer);
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap()
    });

    Ok(())
}

pub async fn client_say(msg: &str) -> Result<String> {
    let mut client = EchoServiceClient::connect("http://[::1]:5001").await?;
    let request = Request::new(EchoRequest {
        message: msg.into(),
    });

    let response = client.echo(request).await?;
    Ok(response.into_inner().message)
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        start_server().await.unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        let msg = "Hello world!";
        let res = client_say(msg).await.unwrap();
        assert_eq!(res, msg);
    }
}
