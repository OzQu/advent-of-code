mod elf_lib;
use elf_lib::{find_elf_with_most_calories, read_elfs, sort_groups_by_sum};
fn main() {
    let file_name = "./data.txt";
    let elfs = read_elfs(file_name);
    match elfs {
        Ok(elfs) => {
            // Day1 a
            let max_elf = find_elf_with_most_calories(&elfs);
            println!("Elf {} had the most calories: {}", max_elf.0 + 1, max_elf.1);

            // Day1 b
            let sorted_elfs = sort_groups_by_sum(elfs);
            let top_three: Vec<_> = sorted_elfs.iter().take(3).collect();
            for group in &top_three {
                let sum = group.iter().sum::<i32>();
                println!("Sum of group {:?} is: {}", group, sum);
            }
            let top_three_sum: i32 = top_three.iter().map(|g| g.iter().sum::<i32>()).sum();

            println!("Sum of the top three group sums: {}", top_three_sum);
        },
        Err(err) => {
            println!("error: {}", err);
        }
    }

}