use std::collections::BTreeSet;

pub fn unique<T: Ord>(xs: Vec<T>) -> Vec<T> {
  let mut set = BTreeSet::new();

  for x in xs {
    set.insert(x);
  }

  // NOTE: we could also use set.into_iter().collect()
  let mut out = Vec::with_capacity(set.len());

  for x in set.into_iter().rev() {
    out.push(x);
  }

  out
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;
  use proptest::prelude::*;

  #[test]
  fn empty_list_returns_empty_list() {
    assert_eq!(Vec::<i32>::new(), unique(vec![]));
  }

  proptest! {
    #[test]
    fn elements_are_returned_in_descending_order(xs in proptest::collection::vec(0..=20, 100)) {
      let actual = unique(xs);

      for i in 1..actual.len() {
        assert!(actual[i-1] >= actual[i], "{} >= {}", actual[i-1], actual[i]);
      }
    }

    #[test]
    fn removes_duplicates(xs in proptest::collection::vec(0..=20, 100)) {
      let actual = unique(xs);

      let mut ocurrences = HashSet::new();

      for x in actual.into_iter() {
        assert_eq!(None, ocurrences.get(&x));
        ocurrences.insert(x);
      }
    }
  }
}
