
/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = demo::add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: u32, b: u32) -> u32 {
  a + b
}

fn div(a: f64, b: f64) -> f64 {
  match b {
    0.0 => panic!("should not be zero"),
    _ => a / b,
  }
}

#[cfg(test)]
mod add_test {
  use super::*;

  #[test]
  fn it_should_add_two_values() {
    assert_eq!(add(1, 2), 3);
  }

  #[test]
  fn it_fails() {
    assert_eq!(add(0, 2), 2);
  }

  #[test]
  fn div_not_zero_should_work() {
    assert_eq!(div(2.0, 1.0), 2.0);
  }


  #[test]
  #[should_panic(expected = "should not be zero")]
  fn div_zero_should_panic() {
    div(2.0, 0.0);
  }

}
