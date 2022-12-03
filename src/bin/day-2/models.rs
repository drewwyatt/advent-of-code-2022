use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {
  InvalidInput,
}

pub struct Tournament {
  rounds: Vec<Round>,
}

impl Tournament {
  pub fn new(rounds: Vec<Round>) -> Self {
    Self { rounds }
  }

  pub fn score(&self) -> usize {
    self.rounds.iter().map(|r| r.score()).sum()
  }
}

pub struct Round {
  opponent: Shape,
  my: Shape,
}

impl Round {
  pub fn score(&self) -> usize {
    self.my.score() + self.my.outcome(&self.opponent).score()
  }
}

impl FromStr for Round {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let halves = s.split(" ").collect::<Vec<&str>>();
    let opponent = Shape::from_str(halves.first().unwrap())?;
    let my = Shape::from_str(halves.last().unwrap())?;

    Ok(Self { opponent, my })
  }
}

enum Outcome {
  Win,
  Lose,
  Draw,
}

impl Outcome {
  pub fn score(&self) -> usize {
    match self {
      Self::Win => 6,
      Self::Lose => 0,
      Self::Draw => 3,
    }
  }
}

#[derive(PartialEq, Eq)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

impl Shape {
  pub fn outcome(&self, opponent: &Shape) -> Outcome {
    match (self, opponent) {
      (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) => {
        Outcome::Win
      }
      (l, r) if l == r => Outcome::Draw,
      _ => Outcome::Lose,
    }
  }

  pub fn score(&self) -> usize {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }
}

impl FromStr for Shape {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" | "X" => Ok(Self::Rock),
      "B" | "Y" => Ok(Self::Paper),
      "C" | "Z" => Ok(Self::Scissors),
      _ => Err(AdventError::InvalidInput),
    }
  }
}
