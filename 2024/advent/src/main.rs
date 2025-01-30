mod day1;
mod day2;
mod errors;
mod utils;

use std::result;

use crate::day1::{process_1, process_2, read_and_parse_file as day1_read_and_parse_file};
use crate::day2::{count_safe_levels, read_and_parse_file as day2_read_and_parse_file};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("1"))]
    day: String,
}

fn main() {
    let args = Args::parse();

    match args.day.as_str() {
        "1" => run_day1(),
        "2" => run_day2(),
        _ => println!("Day {} is not implemented yet", args.day),
    }
}

fn run_day1() {
    let file_content = day1_read_and_parse_file("data/input_1.txt");
    match process_1(file_content) {
        Ok(i32) => {
            println!("Success! Total distance: {}", i32);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let file_content = day1_read_and_parse_file("data/input_1.txt");
    match process_2(file_content) {
        Ok(i32) => {
            println!("Success! Total similarity: {}", i32);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn run_day2() {
    let reports = day2_read_and_parse_file("data/input_2.txt");
    let mut result1: String = String::new();
    let mut result2: String = String::new();
    match count_safe_levels(reports, 1) {
        Ok(u32) => {
            println!("Success! Total safe reports: {}", u32);
            result1 = u32.to_string();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let reports = day2_read_and_parse_file("data/input_2.txt");
    match count_safe_levels(reports, 2) {
        Ok(u32) => {
            println!("Success! Total safe reports: {}", u32);
            result2 = u32.to_string();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("Results: {}, {}", result1, result2);
}
