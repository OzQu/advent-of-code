use std::path::Path;

mod rucksack_lib;
use rucksack_lib::{ calculate_duplicate_points, three_elf_badge_counts };

fn main() {
    let file_path = Path::new("./data.txt");
    let result = calculate_duplicate_points(&file_path);
    println!("Total sum: {:?}", result);

    let result = three_elf_badge_counts(file_path).expect("Error getting badge count");
    println!("Total badge sum: {:?}", result);
}

#[test]
fn test_main() {
    let filepath = Path::new("./test_data/example_data.txt");
    let expected_priority_sum = 157;

    let output = rucksack_lib::calculate_duplicate_points(&filepath);
    assert_eq!(output, expected_priority_sum);
}
