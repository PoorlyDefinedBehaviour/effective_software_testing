//! Roman numerals were created in Ancient Rome and they were used throughout the
//! whole Empire. The numbers were represented by seven different symbols, listed in the following
//! table:
//! I, , 1, (one)unus
//! V, , 5 (five)quinque
//! X, , 10 (ten)decem
//! L, , 50 (fifty)quinquaginta
//! C, , 100 (one hundred)centum
//! D, , 500 (five hundred)quingenti
//! M, , 1,000 (one thousand)
//!
//! To represent all possible numbers, the Romans combined these symbols, following these two
//! rules:
//! Digits of lower or equal value at the right will be added to the higher value digit.
//! Digits of lower value at the left will be subtracted from the higher value digit.
//!
//! There is still one other rule: no symbol can be repeated more than 3 times in a row. For example,
//! the number 4 is represented by the number (5 - 1) and not by .IV IIII
//!
//! Implement a program that receives a Roman numeral (as a string) and
//! returns its representation in our numerical system (as an integer).

pub fn roman_to_i64(number: &str) -> i64 {
  let mut sum = 0;
  let mut previous = 0;

  for character in number.chars().rev() {
    let n = match character {
      'I' => 1,
      'V' => 5,
      'X' => 10,
      'L' => 50,
      'C' => 100,
      'D' => 500,
      'M' => 1000,
      _ => unreachable!(),
    };

    // Since we are traversing the string from the end to the start
    // if we have a number like IV, `n` would be less than previous (`V`)
    // so we know we it is a subtraction notation.
    if n < previous {
      sum -= n;
    } else {
      sum += n;
    }

    previous = n;
  }

  sum
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_roman_to_i64() {
    let tests = vec![
      ("I", 1),
      ("V", 5),
      ("X", 10),
      ("L", 50),
      ("C", 100),
      ("D", 500),
      ("M", 1000),
      ("II", 2),
      ("III", 3),
      ("VI", 6),
      ("XVIII", 18),
      ("XXIII", 23),
      ("DCCLXVI", 766),
      ("IV", 4),
      ("XIV", 14),
      ("XL", 40),
      ("XLI", 41),
      ("CCXCIV", 294),
    ];

    for (input, expected) in tests {
      assert_eq!(expected, roman_to_i64(input), "input={}", input);
    }
  }
}
