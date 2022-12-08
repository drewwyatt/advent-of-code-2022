mod models;
use models::{Groups, Rucksack};

fn main() -> std::io::Result<()> {
  let rucksacks = advent::read_input_for_day_as::<Rucksack>(3)?;
  let sum: usize = rucksacks.iter().map(|s| s.priority()).sum();
  let groups = Groups::from(rucksacks);

  println!("[day-3][part-1] the sum of all item priorities is {}", sum);
  println!(
    "[day-3][part-2] the sum of all item priorities is {}",
    groups.priority()
  );
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;
  use models::Item;

  fn get_rucksacks() -> Vec<Rucksack> {
    vec![
      Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(),
      Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
      Rucksack::from_str("PmmdzqPrVvPwwTWBwg").unwrap(),
      Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
      Rucksack::from_str("ttgJtRGJQctTZtZT").unwrap(),
      Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(),
    ]
  }

  #[test]
  fn day_three_part_one() {
    let rucksacks = get_rucksacks();
    let sum: usize = rucksacks.iter().map(|s| s.priority()).sum();
    assert_eq!(sum, 157);
  }

  #[test]
  fn day_three_part_two() {
    let rucksacks = get_rucksacks();
    let groups = Groups::from(rucksacks);

    assert_eq!(groups.priority(), 70);
  }

  #[test]
  fn priority() {
    let a = Item::new('a');
    let b = Item::new('b');
    let z = Item::new('z');
    let A = Item::new('A');

    assert_eq!(a.priority(), 1);
    assert_eq!(b.priority(), 2);
    assert_eq!(z.priority(), 26);
    assert_eq!(A.priority(), 27);
  }
}
