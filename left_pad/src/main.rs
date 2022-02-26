use std::fmt::Write;

pub fn left_pad(s: &str, desired_string_size: usize, padding: &str) -> String {
  if padding.is_empty() {
    return s.to_string();
  }

  let mut buffer = String::new();

  while buffer.len() + s.len() + padding.len() <= desired_string_size {
    write!(&mut buffer, "{}", padding).expect("unable to add padding to buffer");
  }

  write!(&mut buffer, "{}", s).expect("unable to add string to buffer");

  buffer
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn string_is_returned_when_padding_is_empty(s: String, desired_string_size:usize) {
      assert_eq!(s, left_pad(&s, desired_string_size, ""));
    }

    #[test]
    fn string_is_returned_when_desired_string_size_is_less_than_string_size(s: String, padding: String) {
      prop_assume!(!s.is_empty());
      assert_eq!(s, left_pad(&s, s.len() - 1, &padding));
    }
  }

  #[test]
  fn pads_string() {
    assert_eq!("-abc", left_pad("abc", 4, "-"));
    assert_eq!("--abc", left_pad("abc", 5, "-"));
    assert_eq!("helloabc", left_pad("abc", 8, "hello"))
  }
}
