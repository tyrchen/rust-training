use std::mem;
use std::ops::*;

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

pub trait Value: Mul<Output = Self> + Copy {}

impl Value for f64 {}
// trait
trait Area<T: Value> {
  fn area(&self) -> T;
}

// 1, 2, 3, 4, 5
// n

// y = f(T) scope of T
struct Rectangle<T> {
  width: T,
  height: T,
}

struct Circle<T> {
  radius: T,
}

struct Square<T> {
  width: T,
}

impl<T: Value> Area<T> for Rectangle<T> {
  fn area(&self) -> T {
    self.width * self.height
  }
}

impl<T: Value> Area<T> for Circle<T> {
  fn area(&self) -> T {
    self.radius * self.radius
  }
}

impl<T: Value> Area<T> for Square<T> {
  fn area(&self) -> T {
    self.width * self.width
  }
}

fn calc_area<T: Value>(shape: &Area<T>) -> T {
  shape.area()
}

fn main() {
  // show_array();
  // show_slice();
  // show_tuple();

  // let mut dev: Employee = Default::default();
  // dev.age = 38;
  // println!("{:?}", dev);
  let rec: Rectangle<f64> = Rectangle {
    width: 10.0,
    height: 20.0,
  };
  let circle: Circle<f64> = Circle { radius: 10.0 };
  let square: Square<f64> = Square { width: 10.0 };
  println!(
    "rec area is {}, circle area is {}, square area is {}",
    calc_area(&rec),
    calc_area(&circle),
    calc_area(&square),
  );
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
  let mut array: [u16; 5] = [0, 1, 2, 3, 4];
  for i in array.iter() {
    println!("{}", i);
  }

  let value: Vec<u16> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  println!("{:?}", value);

  println!(
    "array {:?}, vector {:?}",
    mem::size_of_val(&array),
    mem::size_of_val(&value)
  );

  // slice
  let slice2 = &mut array[3..4];
  let slice1 = &mut array[0..3];

  println!("{:?} size: {}", slice1, mem::size_of_val(&slice1));
}

// fn show_array() {
//   let array: [u32; 5] = [0, 1, 2, 3, 5];
//   println!("{:?}, size: {}", array, mem::size_of_val(&array));
//   for i in array.iter() {
//     println!("{}", i);
//   }
// }

// fn show_slice() {
//   let array: [i32; 5] = [0, 1, 2, 3, 4];

//   let slice = &array[0..4];
//   println!("{:?} {}", slice, mem::size_of_val(&slice));
//   for x in slice {
//     println!("x is {}", x);
//   }
// }
