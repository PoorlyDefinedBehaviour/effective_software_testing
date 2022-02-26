use std::collections::LinkedList;

pub fn add_two_numbers(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
  if left.is_empty() || right.is_empty() {
    return vec![];
  }

  left.reverse();
  right.reverse();

  let mut result = LinkedList::new();

  let mut carry = 0;

  for i in 0..std::cmp::max(left.len(), right.len()) {
    let left_digit = if left.len() > i { left[i] } else { 0 };

    let right_digit = if right.len() > i { right[i] } else { 0 };

    if !(0..=9).contains(&left_digit) || !(0..=9).contains(&right_digit) {
      unreachable!()
    }

    let sum = left_digit + right_digit + carry;

    result.push_front(sum % 10);

    carry = sum / 10;
  }

  if carry > 0 {
    result.push_front(carry);
  }

  result.into_iter().collect()
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_lists_should_return_empty_list() {
    assert_eq!(Vec::<i32>::new(), add_two_numbers(vec![], vec![]));
    assert_eq!(Vec::<i32>::new(), add_two_numbers(vec![], vec![1]));
    assert_eq!(Vec::<i32>::new(), add_two_numbers(vec![1], vec![]));
    assert_eq!(Vec::<i32>::new(), add_two_numbers(vec![], vec![1, 2, 3]));
    assert_eq!(Vec::<i32>::new(), add_two_numbers(vec![1, 2, 3], vec![]));
  }

  #[test]
  fn should_add_numbers_with_a_single_digit() {
    assert_eq!(vec![3], add_two_numbers(vec![1], vec![2]));
    assert_eq!(vec![1, 1], add_two_numbers(vec![9], vec![2]));
  }

  #[test]
  fn should_add_numbers_with_many_digits() {
    assert_eq!(vec![5, 5], add_two_numbers(vec![2, 2], vec![3, 3]));
    assert_eq!(vec![5, 2], add_two_numbers(vec![2, 9], vec![2, 3]));
    assert_eq!(vec![4, 7, 6], add_two_numbers(vec![2, 9, 3], vec![1, 8, 3]));
    assert_eq!(vec![4, 4, 7], add_two_numbers(vec![1, 7, 9], vec![2, 6, 8]));
    assert_eq!(
      vec![3, 7, 3, 3, 2],
      add_two_numbers(vec![1, 9, 1, 7, 1], vec![1, 8, 1, 6, 1])
    );
  }

  #[test]
  fn should_add_numbers_with_digits_of_different_lengths() {
    assert_eq!(vec![2, 5], add_two_numbers(vec![2, 2], vec![3]));
    assert_eq!(vec![2, 5], add_two_numbers(vec![3], vec![2, 2]));
    assert_eq!(vec![3, 1], add_two_numbers(vec![2, 2], vec![9]));
    assert_eq!(vec![3, 1], add_two_numbers(vec![9], vec![2, 2]));
  }
}
