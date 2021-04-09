#[derive(Debug)]
struct Ipv4Addr {
  value: String,
}

enum IpAddress {
  Ipv4(Ipv4Addr),
  Ipv6(String),
}

use IpAddress::{Ipv4, Ipv6};

pub fn run() {
  let ip = Ipv4(Ipv4Addr {
    value: String::from("127.0.0.1"),
  });

  match ip {
    Ipv4(address) => println!("this is an ipv4: {:?}", address),
    Ipv6(address) => println!("this is an ipv6: {}", address),
    _ => println!("Not a valid IP"),
  }
}
