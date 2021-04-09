use Linklist::{Item, Nil};

use std::mem::size_of_val;

#[derive(Debug)]
enum Linklist {
  Item(Box<u32>, Box<Linklist>),
  Nil,
}

pub fn run() {
  let value = Box::new(8);
  let node1 = Item(value, Box::new(Nil));
  let node2 = Item(Box::new(7), Box::new(node1));

  println!("node is {:?}, size: {}", node2, size_of_val(&node2));
}
