use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("./test_data/example_data.txt").expect("Unable to open input.txt");
    let reader = BufReader::new(file);

    let (stacks, moves) = read_input(reader);

    let rearranged_stacks = rearrange_crates(stacks, moves);
    let top_crates = get_top_crates(rearranged_stacks);

    println!("Top crates of each stack: {}", top_crates);
}

/*
This function reads the input file and returns two values: the initial
stacks of crates and the list of moves. The stacks are represented as
a vector of VecDeque<char>, while the moves are represented as a vector
of tuples containing the number of crates to move, the source stack index,
and the destination stack index.

The input file is expected to have the following format:
1. The first line contains the initial configuration of crate stacks,
   where each stack is represented by a sequence of characters (crates),
   and stacks are separated by whitespace.
2. The subsequent lines represent the moves to be performed, with each line
   formatted as follows:
   "move <number_of_crates> from <source_stack> to <destination_stack>"
   where <number_of_crates>, <source_stack>, and <destination_stack>
   are integer values.

The function parses the input by:
1. Reading the first line to obtain the initial configuration of crate stacks.
   Each stack is split by whitespace and converted into a VecDeque<char>.
2. Reading the remaining lines to obtain the list of moves. Each line is
   split by whitespace, and the relevant values (number of crates, source
   stack, and destination stack) are extracted, parsed, and stored as tuples
   in a vector.
*/
fn read_input<R: BufRead>(mut reader: R) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks = Vec::new();
    let mut moves = Vec::new();

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read initial stack line");
    let stacks_line = line.trim();

    for stack_str in stacks_line.split_whitespace() {
        let stack: VecDeque<char> = stack_str.chars().collect();
        stacks.push(stack);
    }

    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
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

