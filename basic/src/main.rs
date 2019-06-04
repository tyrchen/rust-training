fn main() {
  // previous owner
  let age = 37u128; // copy / clone trait
  let first = String::from("Tyr"); // type inference "Tyr" => string section 0x32756000
  let last = String::from("Chen");
  let name = full_name(&first, &last);
  let languages = vec!["elixir", "python", "js", "C", "clojure", "rust"];
  println!("Hello, {} with age {}, knows {:?}!", name, age, languages); // borrow again
}

fn full_name(first: &String, last: &String) -> String {
  format!("{} {}", first, last)
} // first / last be freed
