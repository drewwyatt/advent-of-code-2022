use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {
  InvalidInput,
}

pub struct Tournament {
  rounds: Vec<usize>,
}

impl Tournament {
  pub fn part_1(rounds: Vec<Part1Round>) -> Self {
    Self {
      rounds: rounds.iter().map(|r| r.score()).collect(),
    }
  }

  pub fn part_2(rounds: Vec<Part2Round>) -> Self {
    Self {
      rounds: rounds.iter().map(|r| r.score()).collect(),
    }
  }

  pub fn score(&self) -> usize {
    self.rounds.iter().sum()
  }
}

pub struct Part1Round {
  opponent: Shape,
  my: Shape,
}

impl Part1Round {
  pub fn score(&self) -> usize {
    self.my.score() + self.my.outcome(&self.opponent).score()
  }
}

impl FromStr for Part1Round {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let halves = s.split(" ").collect::<Vec<&str>>();
    let opponent = Shape::from_str(halves.first().unwrap())?;
    let my = Shape::from_str(halves.last().unwrap())?;

    Ok(Self { opponent, my })
  }
}

impl Part2Round {
  pub fn score(&self) -> usize {
    self.my.score() + self.my.outcome(&self.opponent).score()
  }
}

pub struct Part2Round {
  opponent: Shape,
  my: Shape,
}

impl FromStr for Part2Round {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let halves = s.split(" ").collect::<Vec<&str>>();

    let opponent = Shape::from_str(halves.first().unwrap())?;
    let outcome = Outcome::from_str(halves.last().unwrap())?;
    let my = outcome.to_shape(&opponent);

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

  pub fn to_shape(&self, opponent: &Shape) -> Shape {
    match self {
      Self::Win => opponent.to_winning(),
      Self::Lose => opponent.to_losing(),
      Self::Draw => *opponent,
    }
  }
}

impl FromStr for Outcome {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "X" => Ok(Self::Lose),
      "Y" => Ok(Self::Draw),
      "Z" => Ok(Self::Win),
      _ => Err(AdventError::InvalidInput),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

  pub fn to_winning(&self) -> Self {
    match self {
      Self::Rock => Self::Paper,
      Self::Paper => Self::Scissors,
      Self::Scissors => Self::Rock,
    }
  }

  pub fn to_losing(&self) -> Self {
    match self {
      Self::Rock => Self::Scissors,
      Self::Paper => Self::Rock,
      Self::Scissors => Self::Paper,
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
