pub fn test_lib() -> String {
  "Hello World".into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(test_lib(), "Hello World".to_string());
  }
}
