use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file1 = File::open("data.txt").expect("Unable to open input.txt");
    let fully_contained = part_one(file1);

    let file2 = File::open("data.txt").expect("Unable to open input.txt");
    let overlapping_pairs = part_two(file2);

    println!(
        "Number of assignment pairs with full containment: {}",
        fully_contained
    );

    println!(
        "Number of overlapping assignment pairs: {}",
        overlapping_pairs
    );
}

fn part_one(file: File) -> u32 {
    let reader = BufReader::new(file);
    count_fully_contained_pairs(reader)
}

fn part_two(file: File) -> u32 {
    let reader = BufReader::new(file);
    count_overlapping_pairs(reader)
}

fn count_fully_contained_pairs<R: BufRead>(reader: R) -> u32 {
    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let assignments: Vec<_> = line
            .split(',')
            .map(|range| {
                let mut parts = range.split('-');
                let start = parts.next().unwrap().trim().parse::<u32>().unwrap();
                let end = parts.next().unwrap().trim().parse::<u32>().unwrap();
                (start, end)
            })
            .collect();

        let (start1, end1) = assignments[0];
        let (start2, end2) = assignments[1];

        if (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1) {
            count += 1;
        }
    }

    count
}

fn count_overlapping_pairs<R: BufRead>(reader: R) -> u32 {
    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let assignments: Vec<_> = line
            .split(',')
            .map(|range| {
                let mut parts = range.split('-');
                let start = parts.next().unwrap().trim().parse::<u32>().unwrap();
                let end = parts.next().unwrap().trim().parse::<u32>().unwrap();
                (start, end)
            })
            .collect();

        let (start1, end1) = assignments[0];
        let (start2, end2) = assignments[1];

        if (start1 <= end2 && end1 >= start2) || (start2 <= end1 && end2 >= start1) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    /**
     * These test cases open the test_data.txt file and call part_one and part_two functions with the file. Update the expected count values in the assert_eq! macro based on the correct results for your test data.
     */
    #[test]
    fn test_part_one() {
        let file = File::open("./test_data/example_data.txt").expect("Unable to open test_data.txt");
        let count = part_one(file);
        assert_eq!(count, 2); // Update this value based on your test_data.txt
    }
    #[test]
    fn test_part_two() {
        let file = File::open("./test_data/example_data.txt").expect("Unable to open test_data.txt");
        let count = part_two(file);
        assert_eq!(count, 4); // Update this value based on your test_data.txt
    }

    /**
     * This test function checks whether the count_fully_contained_pairs function
     * correctly calculates the number of assignment pairs with full containment
     * for the given input string. It creates a Cursor as an in-memory buffer for
     * the input string and passes it to the count_fully_contained_pairs function.
     * The test asserts that the returned count should be equal to 2,
     * which is the expected result for this input.
     */
    #[test]
    fn test_count_fully_contained_pairs() {
        let input =
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";
        let cursor = Cursor::new(input);
        let count = count_fully_contained_pairs(cursor);
        assert_eq!(count, 2);
    }

    /**
     * test_no_contained_pairs checks the case where there are no assignment pairs
     * with full containment. The expected result is 0.
     */
    #[test]
    fn test_no_contained_pairs() {
        let input = 
            "1-3,4-6
            7-9,10-12";
        let cursor = Cursor::new(input);
        let count = count_fully_contained_pairs(cursor);
        assert_eq!(count, 0);
    }

    /**
     * test_all_contained_pairs checks the case where all assignment pairs have full
     * containment. The expected result is 3.
     */
    #[test]
    fn test_all_contained_pairs() {
        let input =
            "1-3,1-4
            3-6,1-6
            8-10,8-12";
        let cursor = Cursor::new(input);
        let count = count_fully_contained_pairs(cursor);
        assert_eq!(count, 3);
    }

    /**
     * test_empty_input checks if the function can handle empty input and returns 0,
     * as there are no assignment pairs to process.
     */
    #[test]
    fn test_empty_input() {
        let input = "";
        let cursor = Cursor::new(input);
        let count = count_fully_contained_pairs(cursor);
        assert_eq!(count, 0);
    }

    /**
     * test_single_pair_input checks the case where there's only one assignment pair
     * in the input. In this example, the first range is fully contained in the second
     * range, so the expected result is 1.
     */
    #[test]
    fn test_single_pair_input() {
        let input = "1-3,1-5";
        let cursor = Cursor::new(input);
        let count = count_fully_contained_pairs(cursor);
        assert_eq!(count, 1);
    }


    /**
     * This test checks whether the count_overlapping_pairs function correctly calculates
     * the number of overlapping assignment pairs for the given input string. The expected
     * result is 4, as mentioned in your assignment description.
     */
    #[test]
    fn test_count_overlapping_pairs() {
        let input =
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";
        let cursor = Cursor::new(input);
        let count = count_overlapping_pairs(cursor);
        assert_eq!(count, 4);
    }

}
