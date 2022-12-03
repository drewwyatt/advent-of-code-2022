mod models;
use std::str::FromStr;

use models::Elves;

fn main() -> std::io::Result<()> {
  let input = advent::read_input_for_day(1)?;
  let elves = Elves::from_str(&input)?;
  println!(
    "The elf carrying the most calories is carrying {} calories",
    elves.most_calories()
  );

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_elves() -> Elves {
    Elves::from_str(
      "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    )
    .unwrap()
  }

  #[test]
  fn part_one() {
    let elves = get_elves();
    assert_eq!(elves.most_calories(), 24000)
  }
}
