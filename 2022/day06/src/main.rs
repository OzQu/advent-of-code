use std::collections::{HashSet, VecDeque};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file_path = "./data.txt";
    let input = read_input_from_file(input_file_path).expect("Failed to read input file");

    if let Some(position) = find_start_of_marker(&input, 4) {
        println!("First start-of-packet marker position: {}", position);
    } else {
        println!("Start-of-packet marker not found.");
    }

    if let Some(position) = find_start_of_marker(&input, 14) {
        println!("First start-of-message marker position: {}", position);
    } else {
        println!("Start-of-message marker not found.");
    }
}

fn read_input_from_file(file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut input = String::new();
    for line in reader.lines() {
        input.push_str(&line?);
    }

    Ok(input)
}

fn find_start_of_marker(input: &str, marker_len: usize) -> Option<usize> {
    let mut unique_chars = VecDeque::new();
    let mut char_set = HashSet::new();

    for (index, ch) in input.chars().enumerate() {
        unique_chars.push_back(ch);
        char_set.insert(ch);

        if unique_chars.len() > marker_len {
            let removed_char = unique_chars.pop_front().unwrap();
            if !unique_chars.contains(&removed_char) {
                char_set.remove(&removed_char);
            }
        }

        if char_set.len() == marker_len {
            return Some(index + 1);
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_marker() {
        let test_cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7), Some(19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5), Some(23)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(6), Some(23)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10), Some(29)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11), Some(26)),
        ];

        for (input, expected_packet_marker, expected_message_marker) in test_cases {
            assert_eq!(
                find_start_of_marker(input, 4),
                expected_packet_marker,
                "Failed for input: {}",
                input
            );

            assert_eq!(
                find_start_of_marker(input, 14),
                expected_message_marker,
                "Failed for input: {}",
                input
            );
        }
    }
}

