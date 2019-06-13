extern crate base64;
extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Cursor;

use base64::{decode, encode, DecodeError};

fn main() -> Result<(), DecodeError> {
  let result = encode("hello world");
  let result1 = &decode(&result).unwrap()[..];
  println!("{}, {:?}", result, result1);
  assert_eq!(b"hello world", result1);

  let mut rdr = Cursor::new(vec![1, 0, 0, 1]);
  // Note that we use type parameters to indicate which kind of byte order
  // we want!
  println!("{:?}", rdr.read_u16::<BigEndian>().unwrap());
  println!("{:?}", rdr.read_u16::<LittleEndian>().unwrap());
  // assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());

  Ok(())
}
