use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum ParseCargoCrateLineError {
    InvalidFormat,
}

fn main() -> () {
    // Read and parse input file
    // let input_file = "./test_data/example_data.txt";
    let input_file = "./data.txt";
    if let Ok(top_chars) = process(input_file) {
        println!("{}", top_chars);
    }
}

fn process(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut stacks, commands) = read_and_parse_input_file(input_file)?;

    // Process commands
    for (num_crates, from, to) in commands {
        // Account for 1-based indexing in commands
        let from_idx = from - 1;
        let to_idx = to - 1;

        // Move crates between stacks
        for _ in 0..num_crates {
            if let Some(crate_char) = stacks[from_idx].pop_front() {
                stacks[to_idx].push_front(crate_char);
            } else {
                eprintln!("Error: Not enough crates in stack {} to move", from);
                return Err("Not enough crates to move".into());
            }
        }
    }

    // Print the final state of the stacks
    println!("Final state of stacks:");
    let mut top_chars = String::new();
    for (idx, stack) in stacks.iter().enumerate() {
        let stack_content: String = stack.iter().collect();
        println!("Stack {}: {}", idx + 1, stack_content);
        if let Some(first) = stack.get(0) {
            top_chars += &first.to_string()
        }
    }

    Ok(top_chars)

}

fn read_and_parse_input_file(file_path: &str) -> Result<(Vec<VecDeque<char>>, Vec<(usize, usize, usize)>), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut cargo_section = true;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut commands: Vec<(usize, usize, usize)> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        match cargo_section {
            true => {
                if line.trim().is_empty() {
                    cargo_section = false;
                } else {
                    match parse_cargo_crate_line(&line, &mut stacks) {
                        Ok(_) => println!("noice"),
                        Err(err) => println!("err: {:?}", err)
                    };
                }
            },
            false => commands.push(parse_command_line(&line))
        }
    }
    
    Ok((stacks, commands))
}

fn parse_cargo_crate_line(line: &str, stacks: &mut Vec<VecDeque<char>>) -> Result<(), ParseCargoCrateLineError> {
    let mut iter = line.chars();
    let mut index = 0;

    while let Some(ch) = iter.next() {
        let stack = match (ch, iter.next(), iter.next()) {
            ('[', Some(c), Some(']')) if c.is_ascii_alphabetic() => {
                let mut deque = VecDeque::new();
                deque.push_front(c);
                Some(deque)
            }
            (' ', Some(' '), Some(' ')) => Some(VecDeque::new()),
            (' ', Some(_), Some(' ')) => None,
            _ => return Err(ParseCargoCrateLineError::InvalidFormat),
        };

        if let Some(stack) = stack {
            if index < stacks.len() {
                if !stack.is_empty() {
                    stacks[index].extend(stack);
                }
            } else {
                stacks.push(stack);
            }

            index += 1;
        }

        // Skip the separator space
        if let Some(next_ch) = iter.next() {
            if next_ch != ' ' {
                return Err(ParseCargoCrateLineError::InvalidFormat);
            }
        }
    }

    Ok(())
}


fn parse_command_line(line: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let num_crates = parts[1].parse::<usize>().unwrap();
    let from = parts[3].parse::<usize>().unwrap();
    let to = parts[5].parse::<usize>().unwrap();

    (num_crates, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read_and_parse_input_file() {
        let test_file_path = Path::new("./test_data/example_data.txt");
    
        let result = read_and_parse_input_file(test_file_path.to_str().unwrap());
    
        assert!(result.is_ok(), "Error when reading and parsing input file");
        let (stacks, commands) = result.unwrap();
    
        // assert stacks
        assert_eq!(3, stacks.len());
        assert_eq!(vec![&'N', &'Z'], stacks[0].iter().collect::<Vec<_>>());
        assert_eq!(vec![&'D', &'C', &'M'], stacks[1].iter().collect::<Vec<_>>());
        assert_eq!(vec![&'P'], stacks[2].iter().collect::<Vec<_>>());
        // assert commands
        let expected_commands = vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ];
        assert_eq!(expected_commands, commands);
    }

    #[test]
    fn test_parse_cargo_crate_line() {
        let input = "    [D] [N]    ";
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        let result = parse_cargo_crate_line(input, &mut stacks);
    
        assert!(result.is_ok());
        assert_eq!(4, stacks.len());
    
        assert!(stacks[0].is_empty());
        assert_eq!(1, stacks[1].len());
        assert_eq!(vec![&'D'], stacks[1].iter().collect::<Vec<&char>>());
        assert_eq!(1, stacks[2].len());
        assert_eq!(vec![&'N'], stacks[2].iter().collect::<Vec<&char>>());
        assert!(stacks[3].is_empty());
    }

    #[test]
    fn test_parse_command_line() {
        let line = "move 2 from 1 to 3";
        let (num_crates, from, to) = parse_command_line(line);

        assert_eq!(num_crates, 2);
        assert_eq!(from, 1);
        assert_eq!(to, 3);
    }

}
