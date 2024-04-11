use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{Result, anyhow};
use std::env;

enum Part {
    Part1,
    Part2,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "part1" => Part::Part1,
            "part2" => Part::Part2,
            _ => Part::Part1, // Default to Part1 if the argument doesn't match
        },
        None => Part::Part1, // Default to Part1 if no argument is provided
    };

    let input_file_path = "./src/data.txt";
    let sum = read_sum_from_file(input_file_path, part).expect("Failed to read input file");
    println!("Calibration value {}", sum);
}

fn read_sum_from_file(file_path: &str, part: Part) -> Result<u32> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut values = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                match part {
                    Part::Part1 => {
                        let value = read_value_part_1(&line)?;
                        values.push(value);
                    },
                    Part::Part2 => {
                        let value = read_value_part_2(&line)?;
                        println!("{} converts to {}", &line, &value);
                        values.push(value);
                    }
                }
            },
            Err(err) => return Err(anyhow!(err))
        }
    }
    let sum = values.iter().sum();

    Ok(sum)
}

fn read_value_part_1(s: &str) -> Result<u32> {
    let d1 = first_digit(s);
    let d2 = last_digit(&s);
    match (d1, d2) {
        (Some(d1), Some(d2)) => {
            Ok((d1.to_string() + &d2.to_string()).parse::<u32>()?)
        },
        _ => Err(anyhow!("Invalid digit"))
    }
}

fn read_value_part_2(s: &str) -> Result<u32> {
    let d1 = first_digit(&convert_digit_words_to_numbers_ltr(&s));
    let d2 = last_digit(&convert_digit_words_to_numbers_rtl(&s));
    match (d1, d2) {
        (Some(d1), Some(d2)) => {
            Ok((d1.to_string() + &d2.to_string()).parse::<u32>()?)
        },
        _ => Err(anyhow!("Invalid digit"))
    }
}

fn convert_digit_words_to_numbers_ltr(s: &str) -> String {
    let word_to_digit = [
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'),
    ];

    let mut result = String::new();
    let mut remaining = s;

    while !remaining.is_empty() {
        let mut found = false;

        for (word, digit) in &word_to_digit {
            if remaining.starts_with(word) {
                result.push(*digit);
                remaining = &remaining[word.len()..];
                found = true;
                break;
            }
        }

        // If no number word is found at the start, move one character forward.
        if !found {
            result.push(remaining.chars().next().unwrap());
            remaining = &remaining[1..];
        }
    }

    result
}

fn convert_digit_words_to_numbers_rtl(s: &str) -> String {
    let word_to_digit = [
        ("nine", '9'), ("eight", '8'), ("seven", '7'), ("six", '6'), 
        ("five", '5'), ("four", '4'), ("three", '3'), ("two", '2'), ("one", '1'),
    ];

    let mut result = String::new();
    let mut chars: Vec<char> = s.chars().collect();
    
    while !chars.is_empty() {
        let mut found = false;

        for (word, digit) in &word_to_digit {
            let word_chars: Vec<char> = word.chars().collect();
            if chars.ends_with(&word_chars) {
                result.push(*digit);
                for _ in 0..word.len() {
                    chars.pop();
                }
                found = true;
                break;
            }
        }

        if !found {
            result.push(chars.pop().unwrap());
        }
    }

    result.chars().rev().collect()
}

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        let digit_in_beginning = "1abc2".to_owned();
        let digit_in_middle = "abc1def2".to_owned();
        let digit_in_end = "abc1".to_owned();
        let multiple_digits = "abc1def2ghi3".to_owned();
        let missing_digit = "abc".to_owned();
        
        assert_eq!(first_digit(&digit_in_beginning), Some('1'));
        assert_eq!(first_digit(&digit_in_middle), Some('1'));
        assert_eq!(first_digit(&digit_in_end), Some('1'));
        assert_eq!(first_digit(&multiple_digits), Some('1'));
        assert_eq!(first_digit(&missing_digit), None);
    }

    #[test]
    fn test_last_digit() {
        let digit_in_end = "1abc2".to_owned();
        let digit_in_middle = "abc12def".to_owned();
        let digit_early = "12abcdef".to_owned();
        let multiple_digits = "1abc2345def67".to_owned();
        let missing_digit = "abc".to_owned();
        
        assert_eq!(last_digit(&digit_in_end), Some('2'));
        assert_eq!(last_digit(&digit_in_middle), Some('2'));
        assert_eq!(last_digit(&digit_early), Some('2'));
        assert_eq!(last_digit(&multiple_digits), Some('7'));
        assert_eq!(last_digit(&missing_digit), None);
    }

    #[test]
    fn test_read_value() {
        let valid_back_to_back_digits = "abc12def".to_owned();
        let valid_separated_digits = "abc1def2".to_owned();
        let valid_missing_second = "abc1".to_owned();
        let missing_both = "abc".to_owned();
        let valid_multiple_digits = "abc1def2ghi3".to_owned();

        assert_eq!(read_value_part_1(&valid_back_to_back_digits).unwrap(), 12);
        assert_eq!(read_value_part_1(&valid_separated_digits).unwrap(), 12);
        assert_eq!(read_value_part_1(&valid_missing_second).unwrap(), 11);
        assert_eq!(format!("{}", read_value_part_1(&missing_both).unwrap_err()), "Invalid digit");
        assert_eq!(read_value_part_1(&valid_multiple_digits).unwrap(), 13);
    }

    #[test]
    fn test_convert_digit_words_to_numbers() {
        let test_strings = [
            ("123fourfive67", "1234567", "1234567"),
            ("eightwothree", "8wo3", "eigh23"),
            ("eighthree", "8hree", "eigh3")
        ];
        for (input, ltr_expected, rtl_expected) in test_strings.iter() {
            assert_eq!(convert_digit_words_to_numbers_ltr(&input), *ltr_expected);
            assert_eq!(convert_digit_words_to_numbers_rtl(&input), *rtl_expected);
        }
    }

    #[test]
    fn test_part2_example_data() {
        let test_rows = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76)
        ];

        for (input, expected) in test_rows.iter() {
            assert_eq!(read_value_part_2(input).unwrap(), *expected);
        }
    }

    #[test]
    fn test_part_1() {
        let sum = read_sum_from_file("./src/test_data.txt", Part::Part1).expect("failed to read test_data.txt");
        assert_eq!(142, sum);
    }

    #[test]
    fn test_part_2() {
        let sum = read_sum_from_file("./src/part2_test_data.txt", Part::Part2).expect("failed to read test_data.txt");
        assert_eq!(281, sum);
    }
}
