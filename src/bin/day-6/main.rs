mod models;

use models::{find_start_of_message_marker, find_start_of_packet_marker, AdventError};

fn main() -> Result<(), AdventError> {
  let input = advent::read_input_for_day(6).map_err(|_| AdventError::CouldNotParseInput)?;
  let marker_1 = find_start_of_packet_marker(&input)?;
  let marker_2 = find_start_of_message_marker(&input)?;

  println!(
    "[day-6][part 1] {} characters were processed to find the start marker",
    marker_1
  );

  println!(
    "[day-6][part 2] {} characters were processed to find the start marker",
    marker_2
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

  #[test]
  fn day_6_part_2() {
    assert_eq!(
      find_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
      19
    );
    assert_eq!(
      find_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(),
      23
    );
    assert_eq!(
      find_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg").unwrap(),
      23
    );
    assert_eq!(
      find_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
      29
    );
    assert_eq!(
      find_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
      26
    );
  }
}
