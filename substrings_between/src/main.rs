pub fn substrings_between<'a, 'b, 'c>(s: &'a str, start: &'b str, end: &'c str) -> Vec<&'a str> {
  let mut substrings = Vec::new();

  if s.is_empty() || start.is_empty() || end.is_empty() {
    return substrings;
  }

  let mut i = 0;

  while i < s.len() {
    let s_subset = &s[i..];

    match s_subset.find(start) {
      // There are not matches in `s`.
      None => break,
      Some(match_starts_at) => {
        match s_subset[match_starts_at..].find(end) {
          // There is a start character but no end character.
          None => break,
          Some(match_ends_at) => {
            substrings
              .push(&s_subset[match_starts_at + start.len()..match_starts_at + match_ends_at]);
            // Jump to the first character that comes after the substring we just matched.
            i += match_starts_at + match_ends_at + start.len();
          }
        }
      }
    }
  }

  substrings
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_matches() {
    assert_eq!(Vec::<&str>::new(), substrings_between("", "a", "b"));
    assert_eq!(Vec::<&str>::new(), substrings_between("a", "", "b"));
    assert_eq!(Vec::<&str>::new(), substrings_between("a", "a", ""));
    assert_eq!(Vec::<&str>::new(), substrings_between("", "", ""));
    assert_eq!(Vec::<&str>::new(), substrings_between("a", "a", "b"));
    assert_eq!(Vec::<&str>::new(), substrings_between("b", "a", "b"));
    assert_eq!(Vec::<&str>::new(), substrings_between("bxxxa", "a", "b"));
  }

  #[test]
  fn one_match() {
    assert_eq!(vec!["1"], substrings_between("a1b", "a", "b"));
    assert_eq!(vec!["xxx"], substrings_between("aaxxxbb", "aa", "bb"));
    assert_eq!(vec!["xxx"], substrings_between("axxxbb", "a", "bb"));
    assert_eq!(vec!["xxx"], substrings_between("aaxxxb", "aa", "b"));
    assert_eq!(vec!["2"], substrings_between("c1ba2ba3c", "a", "b"));
  }

  #[test]
  fn many_matches() {
    assert_eq!(
      vec!["1", "2", "3"],
      substrings_between("a1ba2ba3b", "a", "b")
    );

    assert_eq!(vec!["1", "2"], substrings_between("a1ba2ba3c", "a", "b"));

    assert_eq!(vec!["2", "3"], substrings_between("c1ba2ba3b", "a", "b"));
  }
}
