use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

fn main() {
    let stdin = stdin();
    let (mut stacks, moves) = read_input(stdin.lock());

    rearrange_crates(&mut stacks, &moves);

    for stack in &stacks {
        for c in stack {
            print!("[{}]", c);
        }
        println!();
    }
}

fn read_input<R: BufRead>(reader: R) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks_section: Vec<String> = Vec::new();
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();
    let mut stacks: Vec<VecDeque<char>>;

    let mut in_stacks_section = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            in_stacks_section = false;
            continue;
        }

        if in_stacks_section {
            stacks_section.push(line);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "move" {
                let num_crates = parts[1].parse::<usize>().unwrap();
                let from = parts[3].parse::<usize>().unwrap();
                let to = parts[5].parse::<usize>().unwrap();
                moves.push((num_crates, from, to));
            }
        }
    }

    let num_stacks = stacks_section.last().unwrap().chars().filter(|c| c.is_digit(10)).count();
    stacks = vec![VecDeque::new(); num_stacks];

    for (row_index, line) in stacks_section.iter().enumerate() {
        if row_index == stacks_section.len() - 1 {
            break;
        }

        let mut stack_index = 0;
        let mut char_iter = line.chars().peekable();
        while let Some(c) = char_iter.next() {
            if c.is_alphabetic() {
                stacks[stack_index].push_front(c);
                stack_index += 1;
            } else if c == ' ' && char_iter.peek().map_or(false, |next| *next == ' ') {
                stack_index += 1;
                char_iter.next(); // Consume the extra space
            }
        }
    }

    (stacks, moves)
}

/*
This function iterates through the moves, and for each move, it pops crates
from the source stack and pushes them to the destination stack. The resulting
stacks are returned after all moves are completed. */
fn rearrange_crates(
    stacks: &mut Vec<VecDeque<char>>,
    moves: &[(usize, usize, usize)],
) {
    for (num_crates, from, to) in moves {
        let from = from - 1;
        let to = to - 1;
        for _ in 0..*num_crates {
            let crate_char = stacks[from].pop_front().unwrap();
            stacks[to].push_front(crate_char);
        }
    }
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

