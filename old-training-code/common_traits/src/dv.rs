use std::collections::HashMap;

#[derive(Debug)]
enum Role {
  Admin,
  Designer,
  Developer,
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
  title: Box<str>,
  role: Role,
  skills: HashMap<Box<str>, u8>,
}

pub fn default_trait() {
  let n: u64 = Default::default(); // 0
  let v: Employee = Employee {
    name: "Tyr".to_string(),
    ..Default::default()
  };

  // guess what it is?
  println!("{}, {:?}", n, v);
}
