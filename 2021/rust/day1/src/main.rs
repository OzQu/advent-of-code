use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<usize> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();
    numbers 
}

fn sliding_averages(amount: usize, array: &Vec<usize>) -> Vec<usize> {
    let mut averages = Vec::<usize>::new();
    for (index, _) in array.iter().enumerate() {
        // println!("{:} and {:}", index, value);
        if index >= (amount - 1) {
            let end = index + 1;
            let start: usize = if (end - amount) > 0 { end - amount } else { 0 };
            let sliced = &array[start .. end];
            let reduced = sliced.iter().fold(0, |a, b| a + b);
            averages.push(reduced);
        }
    }
    averages
}

fn count_increased(array: &Vec<usize>) -> usize {
    let mut previous_depth = None;
    let mut increased_count: usize= 0;
    for depth in array {
        match previous_depth {
            None => previous_depth = Some(depth),
            Some(n) => {
                if depth > n {
                    increased_count += 1;
                }
                previous_depth = Some(depth);
            }
        }
    }
    increased_count
}

fn main() {

    let filename = "day1_input.txt";
    let ocean_depths = load_from_file(filename);
    // let ocean_depths = vec![199,200,208,210,200,207,240,269,260,263];
    println!("increased depths: {:?}", count_increased(&ocean_depths));

    let averages = sliding_averages(3, &ocean_depths);
    let averages_increased = count_increased(&averages);
    println!("sliding averages increased: {:?}", averages_increased);
}
