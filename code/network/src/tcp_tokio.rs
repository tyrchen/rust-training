use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub async fn start_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    tokio::spawn(async move {
        loop {
            let (mut stream, addr) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let (reader, mut writer) = stream.split();
                let mut reader = BufReader::new(reader);
                println!("client: {:?}", addr);
                let mut buf = String::new();
                loop {
                    buf.clear();
                    match reader.read_line(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            println!("Server received: {} (bytes: {})", buf, n);
                            writer.write_all(process(&buf).as_bytes()).await.unwrap();
                        }
                        Err(err) => {
                            println!("conn err: {:?}", err);
                            break;
                        }
                    }
                }
            });
        }
    });

    Ok(())
}

pub fn process(req: &str) -> String {
    format!("You sent: {}", req)
}

pub async fn send_msg(stream: &mut TcpStream, msg: &str) -> Result<String> {
    stream.write_all(msg.as_bytes()).await?;
    let mut reader = BufReader::new(stream);

    let mut buf = String::new();
    reader.read_line(&mut buf).await?;
    println!("Client received: {}", buf);

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let addr = "127.0.0.1:3722";
        start_server(addr).await.unwrap();
        let mut stream = TcpStream::connect(addr).await.unwrap();
        let req = "hello world\n";
        let res = send_msg(&mut stream, req).await.unwrap();
        assert_eq!(res, process(req));

        let req = "goodbye world\n";
        let res = send_msg(&mut stream, req).await.unwrap();
        assert_eq!(res, process(req));
    }
}
