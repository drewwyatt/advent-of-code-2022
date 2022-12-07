use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct Rucksack {
  left: Vec<Item>,
  right: Vec<Item>,
  both: Option<Item>,
}

impl Rucksack {
  pub fn new(items: String) -> Self {
    let halfway = items.len() / 2;
    let mut both = None;
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
            both = Some(item);
          }
        }

        return acc;
      });

    Self { left, right, both }
  }

  pub fn priority(&self) -> usize {
    self.both.map_or(0, |i| i.priority())
  }
}

impl FromStr for Rucksack {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Rucksack::new(s.to_owned()))
  }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Item {
  pub id: char,
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
