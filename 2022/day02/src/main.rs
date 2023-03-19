use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod rps_lib;
use rps_lib::{calculate_points_from_line, Strategy};

fn main() {
    let file_name = "./data.txt";
    let scores = calculate_scores(&file_name);
    println!("Part One: Player 1 total score: {}", scores.0.0);
    println!("Part One: Player 2 total score: {}", scores.0.1);
    println!("Part Two: Player 1 total score: {}", scores.1.0);
    println!("Part Two: Player 2 total score: {}", scores.1.1);
}

fn calculate_scores(file_name: &str) -> ((i32, i32), (i32, i32)) {
    let mut p1_total_score_part_one = 0;
    let mut p2_total_score_part_one = 0;
    let mut p1_total_score_part_two = 0;
    let mut p2_total_score_part_two = 0;

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(line) = line {
                let (p1_score_part_one, p2_score_part_one) = calculate_points_from_line(&line, &Strategy::PartOne).expect("Error calculating points");
                p1_total_score_part_one += p1_score_part_one;
                p2_total_score_part_one += p2_score_part_one;
                let (p1_score_part_two, p2_score_part_two) = calculate_points_from_line(&line, &Strategy::PartTwo).expect("Error calculating points");
                p1_total_score_part_two += p1_score_part_two;
                p2_total_score_part_two += p2_score_part_two;
            }
        }
    }

    ((p1_total_score_part_one, p2_total_score_part_one), (p1_total_score_part_two, p2_total_score_part_two))
}

// read lines from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_calculate_scores() {
    let file_name = "./test_data/example_data.txt";
    let ((p1_total_score_part_one, p2_total_score_part_one), (p1_total_score_part_two, p2_total_score_part_two)) = calculate_scores(file_name);
    assert_eq!(p1_total_score_part_one, 15);
    assert_eq!(p2_total_score_part_one, 15);
    assert_eq!(p1_total_score_part_two, 15);
    assert_eq!(p2_total_score_part_two, 12);
}
