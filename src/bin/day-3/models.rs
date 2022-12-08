use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct Groups {
  value: Vec<Group>,
}

impl Groups {
  fn new() -> Self {
    Self { value: vec![] }
  }

  fn add(&mut self, item: Rucksack) {
    if let Some(group) = self.value.last_mut() {
      if group.is_full() {
        self.value.push(Group::new(item))
      } else {
        group.add(item);
      }
    } else {
      self.value.push(Group::new(item));
    }
  }

  pub fn from(rucksacks: Vec<Rucksack>) -> Self {
    let mut groups = Self::new();
    for rucksack in rucksacks {
      groups.add(rucksack);
    }

    groups
  }

  pub fn priority(&self) -> usize {
    self.value.iter().map(|i| i.priority()).sum()
  }
}

struct Group {
  one: Rucksack,
  two: Option<Rucksack>,
  three: Option<Rucksack>,
}

impl Group {
  pub fn new(one: Rucksack) -> Self {
    Self {
      one: one,
      two: None,
      three: None,
    }
  }

  pub fn is_full(&self) -> bool {
    self.three.is_some()
  }

  pub fn add(&mut self, rucksack: Rucksack) {
    if self.two.is_some() {
      self.three = Some(rucksack)
    } else {
      self.two = Some(rucksack)
    }
  }

  pub fn priority(&self) -> usize {
    let mut priority: usize = 0;
    for item in self.one.items.iter() {
      if self.others_contain(item) {
        priority = item.priority();
      }
    }

    priority
  }

  fn others_contain(&self, item: &Item) -> bool {
    if self.two.is_some() && self.three.is_some() {
      return self.two.as_ref().unwrap().contains(item)
        && self.three.as_ref().unwrap().contains(item);
    }

    false
  }
}

#[derive(Clone)]
pub struct Rucksack {
  items: Vec<Item>,
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

    let concatenated = [&left[..], &right[..]].concat();
    Self {
      items: concatenated,
      both,
    }
  }

  pub fn contains(&self, item: &Item) -> bool {
    self.items.contains(item)
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
