use anyhow::Result;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub fn start_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;

    thread::spawn(move || loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("client: {:?}", addr);
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        thread::spawn(move || {
            let mut buf = String::new();
            loop {
                buf.clear();
                match reader.read_line(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("Server received: {} (bytes: {})", buf, n);
                        stream.write_all(process(&buf).as_bytes()).unwrap();
                    }
                    Err(err) => {
                        println!("conn err: {:?}", err);
                        break;
                    }
                }
            }
        });
    });

    Ok(())
}

pub fn process(req: &str) -> String {
    format!("You sent: {}", req)
}

pub fn send_msg(stream: &mut TcpStream, msg: &str) -> Result<String> {
    stream.write_all(msg.as_bytes())?;

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    println!("Client received: {}", buf);

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let addr = "127.0.0.1:3721";
        start_server(addr).unwrap();
        let mut stream = TcpStream::connect(addr).unwrap();
        let req = "hello world\n";
        let res = send_msg(&mut stream, req).unwrap();
        assert_eq!(res, process(req));

        let req = "goodbye world\n";
        let res = send_msg(&mut stream, req).unwrap();
        assert_eq!(res, process(req));
    }
}
