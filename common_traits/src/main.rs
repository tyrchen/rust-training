use std::fmt;
use std::string::ToString;

mod dv;

#[derive(Debug, Default)]
struct CreditCardInfo {
  number: String,
  exp: String,
  holder: String,
}

#[derive(Debug)]
enum Payment {
  Cash(u32),
  CreditCard(CreditCardInfo),
}

impl Default for Payment {
  fn default() -> Self {
    Payment::Cash(Default::default())
  }
}

impl fmt::Display for CreditCardInfo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} with EXP {} BY {}",
      self.number, self.exp, self.holder
    )
  }
}

impl ToString for Payment {
  fn to_string(&self) -> String {
    format!("{:#?}", self)
  }
}

fn process() {
  let payment1 = Payment::Cash(100);
  let payment2 = Payment::CreditCard(CreditCardInfo {
    number: String::from("11112222334455"),
    exp: String::from("09/20"),
    holder: String::from("TIAN CHEN"),
  });
  println!(
    "payment1: {}, payment2: {}",
    payment1.to_string(),
    payment2.to_string()
  );

  match payment2 {
    Payment::CreditCard(info) => println!("{} {:?}", info, info),
    _ => println!("Not matched!"),
  }

  println!("change 42 to string is: {}", 42.to_string());

  dv::default_trait();

  let payment3: Payment = Default::default();
  println!("default for payment: {:#?}", payment3);
}

#[derive(Clone, Debug)]
struct Pointer {
  x: u32,
  y: u32,
}

fn show_pointer(p: Pointer) -> String {
  format!("{:?}", p)
}

fn copy_clone_test() {
  let pointer = Pointer { x: 10, y: 20 };
  let result = show_pointer(pointer.clone());
  println!("result is {}, origin pointer is {:?}", result, pointer);
}

fn main() {
  let mut iter = [10, 20, 30].iter();

  while let Some(n) = iter.next() {
    println!("got {}", n);
  }
  for n in [10, 20, 30].iter() {
    println!("got {}", n);
  }

  println!("{:?}", [10, 20, 30].iter().map(|x| x + 1));

  fn sum(ii: impl IntoIterator<Item = i32>) -> i32 {
    ii.into_iter().sum()
  }

  println!("{}", sum(0..9));
  println!("{}", sum(vec![1, 2, 3]));
  // cloned() here makes an iterator over i32 from an iterator over &i32
  println!("{}", sum([1, 2, 3].iter().cloned()));
}
