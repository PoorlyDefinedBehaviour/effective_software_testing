use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum PlanningPokerError {
  #[error("planning poker requires at least two players. got {got:?}")]
  NotEnoughPlayers { got: usize },
}

#[derive(Debug, PartialEq)]
struct Estimate {
  player: Player,
  estimate: usize,
}

type Player = String;

fn find_extremes(estimates: &[Estimate]) -> Result<(&Estimate, &Estimate), PlanningPokerError> {
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

  Ok((lowest, highest))
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

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
  fn returns_players_with_lowest_and_highest_estimates_when_estimates_are_the_same() {
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

    assert_eq!(
      Ok((&estimates[0], &estimates[1])),
      find_extremes(&estimates)
    );
  }

  #[test]
  fn returns_players_with_lowest_and_highest_estimates_are_not_the_same() {
    let mut estimates = vec![
      Estimate {
        player: Player::from("player_1"),
        estimate: 4,
      },
      Estimate {
        player: Player::from("player_2"),
        estimate: 2,
      },
    ];

    assert_eq!(
      Ok((&estimates[1], &estimates[0])),
      find_extremes(&estimates)
    );

    estimates.push(Estimate {
      player: Player::from("player_3"),
      estimate: 1,
    });

    assert_eq!(
      Ok((&estimates[2], &estimates[0])),
      find_extremes(&estimates)
    );
  }
}
