use std::collections::{HashSet, VecDeque};

struct BoundedVec {
  data: VecDeque<char>,
  size: usize,
}

impl BoundedVec {
  pub fn new(size: usize) -> Self {
    Self {
      data: VecDeque::with_capacity(size),
      size,
    }
  }

  pub fn add(&mut self, c: char) {
    self.data.push_back(c);
    if self.data.len() > self.size {
      self.data.pop_front();
    }
  }

  pub fn all_unique(&self) -> bool {
    if self.data.len() < self.size {
      return false;
    }

    let mut uniq = HashSet::new();
    self.data.iter().all(|i| uniq.insert(i))
  }
}

#[derive(Debug)]
pub enum AdventError {
  CouldNotParseInput,
  CouldNotFindPacketMarker,
}

pub fn find_start_of_packet_marker(s: &str) -> Result<usize, AdventError> {
  let mut buffer = BoundedVec::new(4);
  let mut result = Err(AdventError::CouldNotFindPacketMarker);

  for (index, el) in s.chars().into_iter().enumerate() {
    buffer.add(el);
    if buffer.all_unique() {
      result = Ok(index + 1);
      break;
    }
  }

  result
}
