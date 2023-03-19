use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{ Display, Formatter };

#[derive(Debug)]
pub enum ParsingError {
    NoCommonChar(String),
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::NoCommonChar(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn new(compartment1: String, compartment2: String) -> Self {
        Rucksack {
            compartment1,
            compartment2,
        }
    }

    fn calculate_points(&self) -> i32 {
        let mut total_sum = 0;
        let comp1 = &self.compartment1;
        let comp2 = &self.compartment2;

        let mut seen = HashSet::new();
        for char in comp1.chars() {
            if seen.insert(char) && comp2.contains(char) {
                total_sum += char as i32 - if char.is_lowercase() { 'a' as i32 - 1 } else { 'A' as i32 - 27 };
            }
        }

        total_sum
    }
}

fn parse_line(line: &str) -> Option<Rucksack> {
    let mid = line.len() / 2;
    let compartment1 = line[..mid].to_string();
    let compartment2 = line[mid..].to_string();

    Some(Rucksack::new(compartment1, compartment2))
}

pub fn calculate_duplicate_points(file_path: &Path) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut points = 0;
    for line in reader.lines().flatten() {
        if let Some(rucksack) = parse_line(&line) {
            points += rucksack.calculate_points();
        }
    }

    points
}

fn find_badge_char(first: &str, second: &str, third: &str) -> Result<char, ParsingError> {
    for c1 in first.chars() {
        if second.contains(c1) && third.contains(c1) {
            return Ok(c1)
        }
    }
    Err(ParsingError::NoCommonChar("No Common Character found".to_string()))
}


pub fn three_elf_badge_counts(file_path: &Path) -> Result<i32, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut count = 0;

    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    loop {
        let line1 = match lines_iter.next() {
            Some(line) => line,
            None => break,
        };

        let line2 = match lines_iter.next() {
            Some(line) => line,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid number of lines")),
        };

        let line3 = match lines_iter.next() {
            Some(line) => line,
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid number of lines")),
        };

        let badge = find_badge_char(&line1, &line2, &line3).expect("Couldn't find badge");
        count += badge as i32 - if badge.is_lowercase() { 'a' as i32 - 1 } else { 'A' as i32 - 27 };
    }

    Ok(count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_elf_badge_counts() {
        let file_path = Path::new("./test_data/example_data.txt");
        let total_count = three_elf_badge_counts(file_path);
        assert!(total_count.is_ok());
        assert_eq!(total_count.unwrap(), 70);
    }

    #[test]
    fn test_find_badge_char() {
        let badge_character = find_badge_char(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg");
        assert!(badge_character.is_ok());
        assert_eq!(badge_character.unwrap(), 'r');

        let badge_character = find_badge_char(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw");
        assert!(badge_character.is_ok());
        assert_eq!(badge_character.unwrap(), 'Z');
    }

    #[test]
    fn test_calculate_points() {
        let rucksack = Rucksack::new("ab".to_string(), "ba".to_string());
        assert_eq!(rucksack.calculate_points(), 3);

        let rucksack = Rucksack::new("abc".to_string(), "def".to_string());
        assert_eq!(rucksack.calculate_points(), 0);

        let rucksack = Rucksack::new("Aa".to_string(), "bB".to_string());
        assert_eq!(rucksack.calculate_points(), 0);

        let rucksack = Rucksack::new("aB".to_string(), "Cc".to_string());
        assert_eq!(rucksack.calculate_points(), 0);

        let rucksack = Rucksack::new("abcde".to_string(), "ABCDe".to_string());
        assert_eq!(rucksack.calculate_points(), 5);

        let rucksack = Rucksack::new("Abcde".to_string(), "ABCDE".to_string());
        assert_eq!(rucksack.calculate_points(), 27);

        let rucksack = Rucksack::new("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string());
        assert_eq!(rucksack.calculate_points(), 16, "first example");

        let rucksack = Rucksack::new("jqHRNqRjqzjGDLGL".to_string(), "rsFMfFZSrLrFZsSL".to_string());
        assert_eq!(rucksack.calculate_points(), 38, "second example");

        let rucksack = Rucksack::new("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string());
        assert_eq!(rucksack.calculate_points(), 42, "third example");

        let rucksack = Rucksack::new("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string());
        assert_eq!(rucksack.calculate_points(), 22, "fourth example");

        let rucksack = Rucksack::new("ttgJtRGJ".to_string(), "QctTZtZT".to_string());
        assert_eq!(rucksack.calculate_points(), 20, "fifth example");

        let rucksack = Rucksack::new("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string());
        assert_eq!(rucksack.calculate_points(), 19, "sixth example");
    }

    #[test]
    fn test_calculate_duplicate_points() {
        let input_file = "./test_data/example_data.txt";
        let expected_result = 157;

        let result = calculate_duplicate_points(Path::new(input_file));

        assert_eq!(result, expected_result);
    }
}
