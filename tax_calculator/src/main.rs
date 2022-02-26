// NOTE: using f64 is likely a problem.
pub fn calculate_tax(value: f64) -> f64 {
  // This is the pre-condition.
  assert!(value >= 0.0);

  // Pretend we are actually calculating something...
  let tax = value;

  // This is the post-condition.
  assert!(tax >= 0.0);

  tax
}

fn main() {
  println!("Hello, world!");
}
