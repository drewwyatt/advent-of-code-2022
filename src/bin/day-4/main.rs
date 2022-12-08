mod models;
use models::Pair;

fn main() -> std::io::Result<()> {
  let pairs = advent::read_input_for_day_as::<Pair>(4)?;
  let part_1 = pairs
    .iter()
    .filter(|p| p.has_contained_pair())
    .collect::<Vec<&Pair>>()
    .len();

  println!(
    "[day-4][part-1] there are {} pairs with assignments that fully overlap.",
    part_1
  );

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_pairs() -> Vec<Pair> {
    vec![
      Pair::from_str("2-4,6-8").unwrap(),
      Pair::from_str("2-3,4-5").unwrap(),
      Pair::from_str("5-7,7-9").unwrap(),
      Pair::from_str("2-8,3-7").unwrap(),
      Pair::from_str("6-6,4-6").unwrap(),
      Pair::from_str("2-6,4-8").unwrap(),
    ]
  }

  #[test]
  fn day_4_part_1() {
    let pairs = get_pairs();
    let n = pairs
      .iter()
      .filter(|p| p.has_contained_pair())
      .collect::<Vec<&Pair>>()
      .len();

    assert_eq!(n, 2);
  }
}
