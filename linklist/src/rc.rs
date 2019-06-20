use Linklist::{Item, Nil};

use std::mem::size_of_val;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Linklist {
  Item(u32, Rc<Linklist>),
  Nil,
}

impl Drop for Linklist {
  fn drop(&mut self) {
    println!("item to be freed: {:?}", self);
  }
}

pub fn run() {
  let node1 = Item(8, Rc::new(Nil));
  let node = Rc::new(Item(7, Rc::new(node1)));
  {
    let list1 = Item(1, Rc::clone(&node));
    let list2 = Item(2, Rc::clone(&node));
    println!("list1: {:?}, list2: {:?}", list1, list2);
  }

  println!("node rc is: {}", Rc::strong_count(&node));
  // println!("node is {:?}, size: {}", node, size_of_val(&node));
}
