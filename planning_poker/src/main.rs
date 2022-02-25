use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
enum PlanningPokerError {
  #[error("planning poker requires at least two players. got {got:?}")]
  NotEnoughPlayers { got: usize },
}

#[derive(Debug, PartialEq, Clone)]
struct Estimate {
  player: Player,
  estimate: usize,
}

type Player = String;

fn find_extremes(
  estimates: &[Estimate],
) -> Result<Option<(&Estimate, &Estimate)>, PlanningPokerError> {
  if estimates.len() < 2 {
    return Err(PlanningPokerError::NotEnoughPlayers {
      got: estimates.len(),
    });
  }

  let mut lowest = &estimates[0];
  let mut highest = &estimates[1];

  for estimate in estimates.iter() {
    if estimate.estimate < lowest.estimate {
      lowest = estimate;
    }
    if estimate.estimate > highest.estimate {
      highest = estimate;
    }
  }

  // Everyone had the same estimates.
  if lowest.estimate == highest.estimate {
    return Ok(None);
  }

  Ok(Some((lowest, highest)))
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
   fn returns_players_with_lowest_and_highest_estimates(mut xs: Vec<usize>) {
      xs.push(1);
      xs.push(usize::MAX);

      let xs: Vec<Estimate> = xs.into_iter().filter(|x|*x > 0).map(|i| Estimate {
        player: i.to_string(),
        estimate:i,
      }).collect();

      let actual = find_extremes(&xs).unwrap().unwrap().clone();

      assert_eq!(
        Estimate{
          player: Player::from("1"),
          estimate: 1,
        },
        actual.0.clone()
      );
      assert_eq!(
        Estimate{
          player: usize::MAX.to_string(),
          estimate: usize::MAX,
        },
        actual.1.clone()
      );
    }
  }

  #[test]
  fn at_least_two_estimates_are_required() {
    let tests = vec![
      (vec![], Err(PlanningPokerError::NotEnoughPlayers { got: 0 })),
      (
        vec![Estimate {
          player: Player::from("player_1"),
          estimate: 2,
        }],
        Err(PlanningPokerError::NotEnoughPlayers { got: 1 }),
      ),
    ];

    for (input, expected) in tests {
      assert_eq!(expected, find_extremes(&input));
    }
  }

  #[test]
  fn returns_no_playes_when_estimates_are_equal() {
    let estimates = vec![
      Estimate {
        player: Player::from("player_1"),
        estimate: 2,
      },
      Estimate {
        player: Player::from("player_2"),
        estimate: 2,
      },
      Estimate {
        player: Player::from("player_3"),
        estimate: 2,
      },
    ];

    assert_eq!(Ok(None), find_extremes(&estimates));
  }
}
