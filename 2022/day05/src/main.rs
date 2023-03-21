use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::iter::repeat;

fn main() {
    let file = File::open("./test_data/example_data.txt").expect("Unable to open input.txt");
    let reader = BufReader::new(file);

    let (stacks, moves) = read_input(reader);

    let rearranged_stacks = rearrange_crates(stacks, moves);
    let top_crates = get_top_crates(rearranged_stacks);

    println!("Top crates of each stack: {}", top_crates);
}

fn read_input<R: BufRead>(reader: R) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut move_commands = Vec::new();

    let mut lines: Vec<String> = Vec::new();
    let mut is_move_section = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            is_move_section = true;
            continue;
        }
        if !is_move_section {
            lines.push(line);
        } else {
            move_commands.push(line);
        }
    }

    let num_stacks = lines
        .last()
        .unwrap()
        .split_whitespace()
        .count();

    stacks = repeat(VecDeque::new()).take(num_stacks).collect();

    for line in lines.iter().rev().skip(1) {
        let chars: Vec<char> = line
            .chars()
            .filter(|&c| c != ' ')
            .collect();
    
        for (stack_idx, &crate_char) in chars.iter().enumerate() {
            if crate_char != '[' && crate_char != ']' {
                let actual_idx = stack_idx - 1;
                stacks[actual_idx].push_back(crate_char);
            }
        }
    }

    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut moves = Vec::new();

    for line in move_commands.iter() {
        if let Some(captures) = move_regex.captures(&line) {
            let num_crates = captures[1].parse::<usize>().unwrap();
            let from = captures[2].parse::<usize>().unwrap() - 1;
            let to = captures[3].parse::<usize>().unwrap() - 1;

            moves.push((num_crates, from, to));
        }
    }

    (stacks, moves)
}

/*
This function iterates through the moves, and for each move, it pops crates
from the source stack and pushes them to the destination stack. The resulting
stacks are returned after all moves are completed. */
fn rearrange_crates(
    mut stacks: Vec<VecDeque<char>>,
    moves: Vec<(usize, usize, usize)>,
) -> Vec<VecDeque<char>> {
    for (num_crates, from, to) in moves {
        for _ in 0..num_crates {
            if let Some(crate_char) = stacks[from].pop_front() {
                stacks[to].push_front(crate_char);
            }
        }
    }
    stacks
}

/*
This function iterates through the stacks, gets the front element
(the top crate) of each stack, and collects them into a String. */
fn get_top_crates(stacks: Vec<VecDeque<char>>) -> String {
    stacks
        .into_iter()
        .filter_map(|stack| stack.front().cloned())
        .collect()
}

