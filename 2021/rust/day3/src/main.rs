use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
let file = File::open(filename)?;
Ok(io::BufReader::new(file).lines())
}

fn file_to_string_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    return io::BufReader::new(File::open(filename)?).lines().collect();
}

fn most_common_nth_bit(n: usize, lines: &Vec<String>) -> char {
    let mut one_in_nth: usize = 0;
    for line in lines {
        if line.chars().nth(n).expect("invalid data") == '1' { one_in_nth += 1; }
    }
    if one_in_nth >= (&lines.len() - one_in_nth) { return '1' }
    return '0'
}

fn least_common_nth_bit(n: usize, lines: &Vec<String>) -> char {
    let mut one_in_nth: usize = 0;
    for line in lines {
        if line.chars().nth(n).expect("invalid data") == '1' { one_in_nth += 1; }
    }
    if one_in_nth >= (&lines.len() - one_in_nth) { return '0' }
    return '1'
}

fn power_consuption(lines: io::Lines<io::BufReader<File>>) -> Result<(usize, usize), ParseIntError> {
let mut g_value = String::default();
let mut e_value = String::default();
let mut one_counts: Vec<isize> = Vec::default();
for line in lines {
    if let Ok(line) = line {
        for (i, character) in line.chars().enumerate() {
            match character {
                '1' => {
                    if one_counts.len() <= i { one_counts.push(1) }
                    else { one_counts[i] += 1 }
                },
                _ => {
                    if one_counts.len() <= i { one_counts.push(0) }
                    else { one_counts[i] -= 1 }
                }
            }
        }
    }
}
println!("{:?}", one_counts);
for value in one_counts.iter() {
    if *value > 0 {
        g_value.push_str("1");
        e_value.push_str("0");
    } else { 
        g_value.push_str("0");
        e_value.push_str("1");
    }
}
let gamma = usize::from_str_radix(&g_value, 2)?;
let epsilon = usize::from_str_radix(&e_value, 2)?;
println!("gamma: {:?}", gamma);
println!("epsilon: {:?}", epsilon);
return Ok((gamma, epsilon));
}

fn ox_gen(lines: &mut Vec<String>, bits: &usize) -> () {
    for n in 0..*bits {
        if lines.len() == 1 { return () }
        let most_common_nth = most_common_nth_bit(n, &lines);
        lines.retain(|value| {
            let keep = value.chars().nth(n).unwrap() == most_common_nth;
            keep
        });
    }
}

fn co2_scrub(lines: &mut Vec<String>, bits: &usize) -> () {
    for n in 0..*bits {
        if lines.len() == 1 { return () }
        let least_common_nth = least_common_nth_bit(n, &lines);
        lines.retain(|value| {
            let keep = value.chars().nth(n).unwrap() == least_common_nth;
            keep
        });
    }
}

fn main() -> Result<(), ParseIntError> {
let filename = "data_input.txt";
if let Ok(lines) = read_lines(filename) {
    let (ref gamma, ref epsilon) = power_consuption(lines)?;
    println!("power: {:?}", *gamma * *epsilon);
}
let mut ox: usize= 0;
let mut co2: usize= 0;
if let Ok(mut lines) = file_to_string_vec(filename) {
    let bits = &lines[0].len();
    ox_gen(&mut lines, bits);
    ox = usize::from_str_radix(&lines[0], 2)?;
    println!("ox gen: {:?}", lines);
}
if let Ok(mut lines) = file_to_string_vec(filename) {
    let bits = &lines[0].len();
    co2_scrub(&mut lines, bits);
    co2 = usize::from_str_radix(&lines[0], 2)?;
    println!("co2 scrubber: {:?}", lines);
}
println!("{}", &ox*&co2);
return Ok(());
}
