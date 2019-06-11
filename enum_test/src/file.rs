use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn run(filename: &str) -> Result<String> {
  let mut file = File::open(filename)?;

  let mut contents: String = Default::default();
  file.read_to_string(&mut contents);

  Ok(contents)
}

// use std::fs::File;
// use std::io::prelude::*;

// pub fn run() -> std::io::Result<()> {
//   let mut file = File::open("foo.txt")?;
//   let mut contents = String::new();
//   file.read_to_string(&mut contents)?;
//   assert_eq!(contents, "Hello, world!");
//   Ok(())
// }
