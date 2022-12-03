use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct Elves {
  elves: Vec<Elf>,
}

impl Elves {
  pub fn most_calories(&self) -> i64 {
    self.elves.iter().fold(0, |most, next| {
      if next.calories > most {
        next.calories
      } else {
        most
      }
    })
  }
}

impl FromStr for Elves {
  type Err = std::io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let data = s.lines().fold(vec![Elf::new()], |mut elves, line| {
      if line.is_empty() {
        elves.push(Elf::new())
      } else {
        let calories = line.parse::<i64>().unwrap();
        let elf = elves.last_mut().unwrap();
        elf.add(calories);
      }

      elves
    });

    Ok(Self { elves: data })
  }
}

pub struct Elf {
  pub calories: i64,
}

impl Elf {
  pub fn new() -> Self {
    Self { calories: 0 }
  }

  pub fn add(&mut self, calories: i64) {
    self.calories += calories;
  }
}
