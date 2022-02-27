// NOTE:
// The original method has this signature:
// public static int indexOf(final int[] array, final int valueToFind, int startIndex)
// in my opinion it does not make sense to take the starting index or to return an int.
// If we want to search in a subset of the list, we can just slice it before calling the method.
pub fn index_of<T, P>(xs: &[T], mut predicate: P) -> Option<usize>
where
  P: FnMut(&T) -> bool,
{
  for (i, x) in xs.iter().enumerate() {
    if predicate(x) {
      return Some(i);
    }
  }

  None
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  #[test]
  fn empty_list() {
    assert_eq!(None, index_of(&vec![], |_: &i32| { unreachable!() }));
  }

  proptest! {
    #[test]
    fn returns_none_when_predicates_always_returns_false(xs: Vec<i32>) {
      assert_eq!(None, index_of(&xs, |_| false));
    }

    #[test]
    fn returns_the_index_of_the_element(mut xs in proptest::collection::vec(0..=100, 100)) {
      let random_index = rand::thread_rng().gen_range(0..=xs.len()-1);

      xs[random_index] = 200;

      assert_eq!(Some(random_index), index_of(&xs, |x| *x == 200));
    }
  }
}
