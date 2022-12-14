use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {
  UnrecognizedInputLine,
}

lazy_static! {
  static ref REARRANGE_STEP: Regex =
    Regex::new(r"^move (?<number>\d+) from (?<origin>\d+) to (?<destination>\d+)$").unwrap();
}

pub struct Crane {
  stacks: Vec<Stack>,
}

impl FromStr for Crane {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for line in s.lines() {
      match line {
        "" => Ok(()),
        _ => Err(AdventError::UnrecognizedInputLine),
      }?;
    }

    Ok(Self { stacks: vec![] })
  }
}

impl Crane {
  fn read_crate_line(line: &str) -> Vec<(usize, char)> {
    todo!()
  }

  fn read_rearrange_step_line(line: &str) {}
}

struct Stack {
  crates: Vec<char>,
}
