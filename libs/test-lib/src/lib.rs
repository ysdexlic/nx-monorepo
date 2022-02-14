pub fn test_lib() -> String {
  "test_lib".into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(test_lib(), "test_lib".to_string());
  }
}
