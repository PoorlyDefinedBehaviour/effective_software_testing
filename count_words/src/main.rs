/// Given a sentence, the program should count the number of words that end
/// with either an "s" or an "r". A word ends when a non-letter appears. The
/// program returns the number of words.
pub fn count_words(s: &str) -> usize {
  let mut count = 0;

  for word in s.split_terminator(|character: char| !character.is_alphabetic()) {
    if word.ends_with('s') || word.ends_with('r') {
      count += 1;
    }
  }

  count
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_matches() {
    assert_eq!(0, count_words(""));
    assert_eq!(0, count_words("hello"));
    assert_eq!(0, count_words("helasarlo"));
    assert_eq!(0, count_words("hello world"));
    assert_eq!(0, count_words("hellaso arworld"));
    assert_eq!(0, count_words("ra sa"));
  }

  #[test]
  fn words_ending_in_s() {
    assert_eq!(1, count_words("s"));
    assert_eq!(1, count_words("xs"));
    assert_eq!(2, count_words("s s"));
    assert_eq!(2, count_words("xs xs"));
    assert_eq!(2, count_words("saasaa xxxs aaaaa zzzzs"));
  }

  #[test]
  fn words_ending_in_r() {
    assert_eq!(1, count_words("r"));
    assert_eq!(1, count_words("xr"));
    assert_eq!(2, count_words("r r"));
    assert_eq!(2, count_words("xr xr"));
    assert_eq!(1, count_words("saasaa xxxr aaaaa"));
    assert_eq!(2, count_words("saasaa xxxr aaaaa zzzzr"));
  }
}
