use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Unable to open input.txt");
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
and the destination stack index. */
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

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let num_crates = parts[1].parse::<usize>().unwrap();
        let from = parts[3].trim_end_matches(',').parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        moves.push((num_crates, from, to));
    }

    (stacks, moves)
}
