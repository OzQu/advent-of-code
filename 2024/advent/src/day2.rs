use crate::utils::read_lines;
use crate::errors::ParseError;
use num::abs;
use std::ops::Sub;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Level {
    value: i32,
}

impl Sub for Level {
    type Output = i32;

    fn sub(self, other: Level) -> i32 {
        self.value - other.value
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Level {
    pub fn is_safe(&self, previous_level: &Level) -> bool {
        self.value == previous_level.value || abs(self.value - previous_level.value) > 3
    }
}

pub struct Report {
    safe: Option<bool>, // to not need reprocessing
    levels: Vec<Level>,
}

impl Report {
    fn is_safe(&mut self, ignore_level_id: Option<usize>) -> bool {
        if let Some(safe) = self.safe {
            return safe;
        }

        if self.levels.is_empty() {
            return false;
        }

        let mut increasing = true;
        let mut decreasing = true;
        let mut not_same_or_not_close = true;

        for i in 1..self.levels.len() {
            let mut previous = i - 1; // start normally by comparing against previous
            if let Some(ignore_level_id) = ignore_level_id {
                if ignore_level_id > self.levels.len() {
                    return false; // Can't ignore something that does not exist
                }
                if i == ignore_level_id {
                    continue; // if this level is to be ignored, skip it
                }
                if i - 1 == ignore_level_id { // if previous level is to be ignored, compare to the one before
                    if i == 1 { // but if the one before does not exist, skip this. This one will be compared in the next loop
                        continue;
                    }
                    previous = i - 2; // If previous is to be ignored, compare to the one before
                }
            }
            //
            // the actual comparisons
            //
            if self.levels[i].is_safe(&self.levels[previous]) {
                // return immediately if adjacent levels are the same or are not within 3
                println!("{}: not safe levels {:?}", i, self.levels);
                not_same_or_not_close = false;
                break;
            }
            // Check if not decreasing or increasing
            if self.levels[i].value > self.levels[previous].value {
                println!("{}: not decreasing {:?}", i, self.levels);
                decreasing = false;
            }
            if self.levels[i].value < self.levels[previous].value {
                println!("{}: not increasing {:?}", i, self.levels);
                increasing = false;
            }
        }

        if (increasing || decreasing) && not_same_or_not_close {
            self.safe = Some(true);
            true
        } else {
            // if not increasing or decreasing, or if levels are the same or not within 3, it is not safe
            if let Some(ignore_level_id) = ignore_level_id {
                return self.is_safe(Some(ignore_level_id + 1));
            }
            self.safe = Some(false);
            false
        }
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Report {{ is_safe: {:?}, levels: {:?} }}", self.safe, self.levels)
    }
}

pub fn count_safe_levels(reports: Result<Vec<Report>, ParseError>, challenge_number: usize) -> Result<u32, ParseError> {
    let ignore_level = match challenge_number {
        1 => None,
        2 => Some(0),
        _ => None,
    };
    let reports = reports?;
    let mut safe_count = 0;
    for mut report in reports {
        println!("Checking report: {}", report);
        if report.is_safe(ignore_level) {
            println!("Report is safe \n");
            safe_count += 1;
        } else {
            println!("Report is not safe \n");
        }
    }
    Ok(safe_count)
}

pub fn read_and_parse_file(file_path: &str) -> Result<Vec<Report>, ParseError> {
    if let Ok(lines) = read_lines(file_path) {
        let mut reports = Vec::new();
        for line in lines {
            let mut levels = Vec::new();
            if let Ok(row) = line {
                for value in row.split_whitespace() {
                    let value: i32 = value.parse().map_err(ParseError::from)?;
                    let level = Level { value };
                    levels.push(level);
                }
            }
            reports.push(Report {
                safe: None,
                levels
            });
        }
        Ok(reports)
    } else {
        return Err(ParseError::ReadError);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_parse_file() {
        let reports = read_and_parse_file("data/test_input_2.txt");
        let safe_count = count_safe_levels(reports, 1).unwrap();
        assert_eq!(safe_count, 2);
    }

    #[test]
    fn test_read_and_parse_file_2() {
        let reports = read_and_parse_file("data/test_input_2.txt");
        let safe_count = count_safe_levels(reports, 2).unwrap();
        assert_eq!(safe_count, 4);
    }
}