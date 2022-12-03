mod models;
use models::{Round, Tournament};

fn main() -> std::io::Result<()> {
  let rounds = advent::read_input_for_day_as::<Round>(2)?;
  let tournament = Tournament::new(rounds);

  println!(
    "[day-2][part-1] your total score would be: {}",
    tournament.score()
  );
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_tournament() -> Tournament {
    let rounds = vec![
      Round::from_str("A Y").unwrap(),
      Round::from_str("B X").unwrap(),
      Round::from_str("C Z").unwrap(),
    ];

    Tournament::new(rounds)
  }

  #[test]
  fn day_two_part_one() {
    let tournament = get_tournament();
    assert_eq!(tournament.score(), 15)
  }
}
