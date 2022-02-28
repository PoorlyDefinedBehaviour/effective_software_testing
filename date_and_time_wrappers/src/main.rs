//! We can create a wrapper type that has operations related to time and date.
//! The wrapper type should be easily mockable to make testing code
//! that depends on a specific time or date easier.

mod clock {
  use chrono::Datelike;

  pub struct Clock;

  #[cfg_attr(test, mockall::automock)]
  impl Clock {
    pub fn month(&self) -> u32 {
      let today = chrono::Utc::now();

      today.month()
    }

    pub fn day(&self) -> u32 {
      let today = chrono::Utc::now();

      today.day()
    }
  }
}

#[mockall_double::double]
use clock::Clock;

use chrono::Datelike;
use std::time;

pub struct ChristmasDiscount {
  clock: Clock,
}

impl ChristmasDiscount {
  pub fn new(clock: Clock) -> Self {
    Self { clock }
  }
}

impl ChristmasDiscount {
  fn is_christmas(&self) -> bool {
    self.clock.month() == 12 && self.clock.day() == 25
  }

  // NOTE: f64 for money = bad.
  pub fn apply_discount(&self, amount: f64) -> f64 {
    let discount_percentage = if self.is_christmas() { 0.15 } else { 0.0 };

    amount - (amount * discount_percentage)
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn does_not_apply_discount_if_today_is_not_christmas() {
    let mut clock = Clock::new();

    clock.expect_month().return_const(12_u32);
    clock.expect_day().return_const(24_u32);

    let sut = ChristmasDiscount::new(clock);

    assert_eq!(10.0, sut.apply_discount(10.0));
  }

  #[test]
  fn applies_discount_if_today_is_christmas() {
    let mut clock = Clock::new();

    clock.expect_month().return_const(12_u32);
    clock.expect_day().return_const(25_u32);

    let sut = ChristmasDiscount::new(clock);
    dbg!(sut.apply_discount(100.0));
    assert_eq!(85.0, sut.apply_discount(100.0));
  }
}
