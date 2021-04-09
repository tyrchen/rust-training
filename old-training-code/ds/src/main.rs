use std::mem;
mod shape;
// by default
// use std::string::String;

#[derive(Debug)]
enum Role {
  Developer,
  Designer,
  Admin,
  Specialist,
}

impl Default for Role {
  fn default() -> Self {
    Role::Developer
  }
}

#[derive(Debug, Default)]
struct Employee {
  name: String,
  age: u8,
  role: Role,
}

fn main() {
  show_array();
  show_slice();
  show_tuple();

  let mut dev: Employee = Default::default();
  dev.age = 38;
  println!("{:?}", dev);

  // shape trait
  shape::static_dispatch();
  shape::dynamic_dispatch();
}

fn show_tuple() {
  let tuple = ("hello", 42u128, "world", [3, 6, 9]);

  println!("First element is {}", tuple.0);
  println!("Second element is {}", tuple.1);
  println!("Third element is {}", tuple.2);
  let mut counter = 0;
  for x in &tuple.3 {
    println!("Element {} of the fourth element is {}", counter, x);
    counter += 1;
  }
}

fn show_array() {
  let array: [u32; 5] = [0, 1, 2, 3, 5];
  println!("{:?}, size: {}", array, mem::size_of_val(&array));
  for i in array.iter() {
    println!("{}", i);
  }
}

fn show_slice() {
  let array: [i32; 5] = [0, 1, 2, 3, 4];

  let slice = &array[0..4];
  println!("{:?} {}", slice, mem::size_of_val(&slice));
  for x in slice {
    println!("x is {}", x);
  }
}
