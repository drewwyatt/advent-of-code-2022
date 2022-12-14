mod models;

use models::{find_start_of_packet_marker, AdventError};

fn main() -> Result<(), AdventError> {
  let input = advent::read_input_for_day(6).map_err(|_| AdventError::CouldNotParseInput)?;
  let marker = find_start_of_packet_marker(&input)?;

  println!(
    "[day-6][part 1] {} characters were processed to find the start marker",
    marker
  );

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_6_part_1() {
    assert_eq!(
      find_start_of_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
      7
    );
    assert_eq!(
      find_start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(),
      5
    );
    assert_eq!(
      find_start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg").unwrap(),
      6
    );
    assert_eq!(
      find_start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
      10
    );
    assert_eq!(
      find_start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
      11
    );
  }
}
