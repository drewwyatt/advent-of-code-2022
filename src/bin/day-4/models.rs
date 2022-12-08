use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct Pair {
  first: Assignment,
  second: Assignment,
}

impl Pair {
  pub fn has_contained_pair(&self) -> bool {
    self.first.contains(&self.second) || self.second.contains(&self.first)
  }

  pub fn has_overlapping_pair(&self) -> bool {
    self.first.overlaps_with(&self.second)
  }
}

impl FromStr for Pair {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let assignments = s
      .split(",")
      .map(|s| Assignment::new(s))
      .collect::<Vec<Assignment>>();

    Ok(Self {
      first: assignments[0],
      second: assignments[1],
    })
  }
}

#[derive(Clone, Copy)]
struct Assignment {
  pub from: i32,
  pub to: i32,
}

impl Assignment {
  pub fn new(assignment: &str) -> Self {
    let splits = assignment
      .split("-")
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();

    Self {
      from: splits[0],
      to: splits[1],
    }
  }

  pub fn contains(&self, other: &Assignment) -> bool {
    self.from <= other.from && self.to >= other.to
  }

  pub fn overlaps_with(&self, other: &Assignment) -> bool {
    let s_range = self.from..=self.to;
    let o_range = other.from..=other.to;

    s_range.contains(&other.from)
      || s_range.contains(&other.to)
      || o_range.contains(&self.from)
      || o_range.contains(&self.to)
  }
}
