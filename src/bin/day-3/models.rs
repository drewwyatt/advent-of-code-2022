use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct Rucksack {
  left: Vec<Item>,
  right: Vec<Item>,
  pub both: Vec<Item>,
}

impl Rucksack {
  pub fn new(items: String) -> Self {
    let halfway = items.len() / 2;
    let mut both = vec![];
    let (left, right) = items
      .chars()
      .enumerate()
      .fold((vec![], vec![]), |mut acc, (idx, c)| {
        if idx < halfway {
          acc.0.push(Item::new(c));
        } else {
          let item = Item::new(c);
          acc.1.push(item);
          if acc.0.contains(&item) {
            both.push(item);
          }
        }

        return acc;
      });

    Self { left, right, both }
  }
}

impl FromStr for Rucksack {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Rucksack::new(s.to_owned()))
  }
}

#[derive(Clone, Copy)]
pub struct Item {
  pub id: char,
}

impl PartialEq for Item {
  fn eq(&self, other: &Item) -> bool {
    self.id == other.id
  }
}

impl Item {
  pub fn priority(&self) -> usize {
    let code = self.id as usize;
    if self.id.is_ascii_uppercase() {
      return code - 38;
    } else {
      return code - 96;
    }
  }

  pub fn new(id: char) -> Self {
    Self { id }
  }
}
