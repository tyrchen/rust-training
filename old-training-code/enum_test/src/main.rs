use std::io::Result;

// mod address;
mod file;
mod hash_map;

fn main() {
  // address::run();
  hash_map::run();

  // println!(
  //   "Result is: {:?}",
  //   file::run("/tmp/hello1").unwrap_or_else(|x| format!("{:?}", x))
  // );
}
