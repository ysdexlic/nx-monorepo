pub fn test_lib_2() -> String {
  "Goodbye World".into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(test_lib_2(), "Goodbye World".to_string());
  }
}
