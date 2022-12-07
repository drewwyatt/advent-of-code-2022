mod models;
use models::Rucksack;

fn main() {}

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
  fn day_three_part_one() {}

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
