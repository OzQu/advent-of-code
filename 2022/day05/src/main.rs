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