mod models;
use models::{Part1Round, Tournament};

use crate::models::Part2Round;

fn main() -> std::io::Result<()> {
  let part_one_rounds = advent::read_input_for_day_as::<Part1Round>(2)?;
  let part_two_rounds = advent::read_input_for_day_as::<Part2Round>(2)?;

  let part_1 = Tournament::part_1(part_one_rounds);
  let part_2 = Tournament::part_2(part_two_rounds);

  println!(
    "[day-2][part-1] your total score would be: {}",
    part_1.score()
  );

  println!(
    "[day-2][part-2] your total score would be: {}",
    part_2.score()
  );
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_tournament() -> Tournament {
    let rounds = vec![
      Part1Round::from_str("A Y").unwrap(),
      Part1Round::from_str("B X").unwrap(),
      Part1Round::from_str("C Z").unwrap(),
    ];

    Tournament::part_1(rounds)
  }

  #[test]
  fn day_two_part_one() {
    let tournament = get_tournament();
    assert_eq!(tournament.score(), 15)
  }
}
