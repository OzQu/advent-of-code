mod errors;

use crate::errors::ParseError;
use num::abs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let file_content = read_and_parse_file("./input.txt");
    match process_1(file_content) {
        Ok(i32) => {
            println!("Success! Total distance: {}", i32);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let file_content = read_and_parse_file("./input.txt");
    match process_2(file_content) {
        Ok(i32) => {
            println!("Success! Total distance: {}", i32);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn process_1(file_content: Result<(Vec<i32>, Vec<i32>), ParseError>) -> Result<i32, ParseError> {
    let (mut left, mut right) = file_content?;
    left.sort();
    right.sort();
    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += abs(l - r)
    }

    Ok(total_distance)
}

fn process_2(file_content: Result<(Vec<i32>, Vec<i32>), ParseError>) -> Result<i32, ParseError> {
    let (left, right) = file_content?;
    let mut right_map = HashMap::new();
    for r in right.iter() {
        *right_map.entry(r).or_insert(0) += 1;
    }
    let mut total_similarity = 0;
    for l in left.iter() {
        if let Some(count) = right_map.get(l) {
            total_similarity += l * count;
        }
    }

    Ok(total_similarity)
}

fn read_and_parse_file(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), ParseError> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let numbers: Vec<i32> =
                    row.split_whitespace().map(|s| s.parse().unwrap()).collect();
                if numbers.len() == 2 {
                    left.push(numbers[0]);
                    right.push(numbers[1]);
                } else {
                    return Err(ParseError::InvalidInput(format!(
                        "expected two numbers per line, but got: {}",
                        row
                    )));
                }
            }
        }
    } else {
        return Err(ParseError::ReadError);
    }

    Ok((left, right))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_parse_file() {
        match read_and_parse_file("./test_input.txt") {
            Ok((left, right)) => {
                assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
                assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
            }
            Err(e) => {
                panic!("Error reading file: {}", e);
            }
        }
    }

    #[test]
    fn test_process_1() {
        let file_content = read_and_parse_file("./test_input.txt");
        match process_1(file_content) {
            Ok(i32) => {
                assert_eq!(i32, 11);
            }
            Err(e) => {
                panic!("Error processing file: {}", e);
            }
        }
    }

    #[test]
    fn test_process_2() {
        let file_content = read_and_parse_file("./test_input.txt");
        match process_2(file_content) {
            Ok(i32) => {
                assert_eq!(i32, 31);
            }
            Err(e) => {
                panic!("Error processing file: {}", e);
            }
        }
    }
}
