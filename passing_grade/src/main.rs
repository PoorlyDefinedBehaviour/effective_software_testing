pub fn passed(grade: f32) -> bool {
  assert!(grade >= 1.0 && grade <= 10.0);

  grade >= 5.0
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
    fn should_return_false(grade in 1.0..=4.9f32) {
      assert_eq!(false, passed(grade));
    }

    #[test]
    fn should_return_true(grade in 5.0..=10.0f32) {
      assert_eq!(true, passed(grade));
    }

    #[test]
    #[should_panic]
    fn invalid_grade_below_the_min_grade(grade in f32::MIN..=0.9) {
      passed(grade);
    }

    #[test]
    #[should_panic]
    fn invalid_grade_above_the_max_grade(grade in 10.1..=f32::MAX) {
      passed(grade);
    }
  }
}
