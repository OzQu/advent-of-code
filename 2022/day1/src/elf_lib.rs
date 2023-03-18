use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum FileParsingError {
    IoError(io::Error)
}

impl fmt::Display for FileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileParsingError::IoError(e) => write!(f, "I/O error: {}", e)
        }
    }
}

impl Error for FileParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileParsingError::IoError(e) => Some(e)
        }
    }
}

#[derive(Debug)]
pub enum LineType {
    CaloryValue(i32),
    EndOfElfData
}

#[derive(Debug)]
pub enum ParseLineError {
    InvalidInteger(ParseIntError)
}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseLineError::InvalidInteger(e) => {
                write!(f, "Failed to parse the line as an i32: {}", e)
            }
        }
    }
}

impl Error for ParseLineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseLineError::InvalidInteger(e) => Some(e)
        }
    }
}

fn parse_line(line: &str) -> Result<LineType, ParseLineError> {
    if line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
        Ok(LineType::EndOfElfData)
    } else {
        match line.parse::<i32>() {
            Ok(number) => Ok(LineType::CaloryValue(number)),
            Err(e) => Err(ParseLineError::InvalidInteger(e))
        }
    }
}

pub fn read_elfs(file_name: &str) -> Result<Vec<Vec<i32>>, FileParsingError> {
    let path = Path::new(file_name);
    let file = File::open(&path).map_err(FileParsingError::IoError)?;

    let reader = io::BufReader::new(file);

    let mut groups: Vec<Vec<i32>> = Vec::new();
    let mut current_group: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(FileParsingError::IoError)?;

        match parse_line(&line) {
            Ok(LineType::CaloryValue(calories)) => {
                current_group.push(calories);
            },
            Ok(LineType::EndOfElfData) => {
                if !current_group.is_empty() {
                    groups.push(current_group);
                    current_group = Vec::new();
                }
            },
            Err(e) => {
                eprintln!("Error parsing line: {}", e);
            }
        }
    }

    // Add the last group if it's not empty
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    Ok(groups)
}

pub fn find_elf_with_most_calories(groups: &Vec<Vec<i32>>) -> (usize, i32) {
    let mut max_calories = std::i32::MIN;
    let mut elf_index = 0;

    for (index, calories) in groups.iter().enumerate() {
        let total_calories: i32 = calories.iter().sum();
        if total_calories > max_calories {
            max_calories = total_calories;
            elf_index = index;
        }
    }

    (elf_index, max_calories)
}

pub fn sort_groups_by_sum(mut groups: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    groups.sort_by_key(|inner| inner.iter().sum::<i32>());
    groups.reverse();
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_read_elfs() {
        let file_name = "./test_data/example_data.txt";
        let file = read_elfs(file_name);
        match file {
            Ok(groups) => {
                // Print the groups
                for (i, group) in groups.iter().enumerate() {
                    println!("Group {}: {:?}", i + 1, group);
                }
                assert_eq!(groups.len(), 5);
                assert_eq!(groups[0].len(), 3);
                assert_eq!(groups[1].len(), 1);
                assert_eq!(groups[2].len(), 2);
                assert_eq!(groups[3].len(), 3);
                assert_eq!(groups[4].len(), 1);
            },
            Err(e) => {
                eprintln!("Error with the file: {}", e);
                assert!(false, "error parsing file");
            }
        }
    }

    #[test]
    fn can_find_most_calories() {
        // groups[2] has the most calories (4)
        let groups = vec![vec![1, 1], vec![2, 1], vec![2, 2], vec![1, 2]];
        let max_calories = find_elf_with_most_calories(&groups);
        assert_eq!(max_calories.0, 2, "Index 2 should have the most calories");
        assert_eq!(max_calories.1, 4, "Index 2 should have 4 calories");
    }

    #[test]
    fn can_sort_by_calories() {
        let groups = vec![
            vec![1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1],
        ];
        let sorted_groups = sort_groups_by_sum(groups);
        assert_eq!(
            sorted_groups,
            vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1],
                vec![1, 1]],
            "sorted incorrectly"
        )
    }
}


