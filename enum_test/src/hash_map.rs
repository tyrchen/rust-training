
use std::collections::HashMap;

pub fn run() {
  let mut map: HashMap<&str, HashMap<&str, HashMap<&str, &str>>> = HashMap::new();

  let mut map_inner: HashMap<&str, &str> = HashMap::new();
  map_inner.insert("hello", "world");

  let mut map_middle: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
  map_middle.insert("greeting", map_inner);

  map.insert("my", map_middle);

  match map.get("my") {
    Some(result) => match result.get("greeting") {
      Some(result1) => match result1.get("hello") {
        Some(result2) => println!("result is {:?}", result2),
        None => println!("None"),
      },
      None => (),

      None => (),
    },
    None => println!("No key found!"),
  }

  println!("map is: {:?}", map);
}
