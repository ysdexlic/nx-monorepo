use test_lib::test_lib;

fn something() -> String {
  "something".into()
}

fn main() {
  println!("Output from test lib: {}", test_lib());
  println!("Output from something(): {}", something());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn something_works() {
    assert_eq!(something(), "something".to_string());
  }
}
