use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{Result, anyhow};
use std::env;

enum Part {
    Part1,
    Part2,
}

struct GameSet {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    game_sets: Vec<GameSet>,
}

struct BagContent {
    red: u32,
    green: u32,
    blue: u32
}

impl GameSet {
    fn new() -> Self {
        GameSet { red: 0, green: 0, blue: 0 }
    }

    fn add(&mut self, color: &str, quantity: u32) {
        match color {
            "red" => self.red += quantity,
            "green" => self.green += quantity,
            "blue" => self.blue += quantity,
            // _ => Err(anyhow!("Invalid color"))
            _ => {}
        }
    }

    fn is_possible(&self, bag: &BagContent) -> bool {
        bag.red >= self.red && bag.green >= self.green && bag.blue >= self.blue
    }
}

impl Game {
    fn is_possible(&self, bag: &BagContent) -> bool {
        self.game_sets.iter().all(|game_set| game_set.is_possible(bag))
    }

    fn fewest_cubes(&self) -> BagContent {
        let mut min_red = u32::MIN;
        let mut min_green = u32::MIN;
        let mut min_blue = u32::MIN;

        for game_set in &self.game_sets {
            min_red = min_red.max(game_set.red);
            min_green = min_green.max(game_set.green);
            min_blue = min_blue.max(game_set.blue);
        }

        BagContent {
            red: min_red,
            green: min_green,
            blue: min_blue,
        }
    }

    fn get_power(&self) -> u32 {
        let fewest_cubes = self.fewest_cubes();
        fewest_cubes.red * fewest_cubes.green * fewest_cubes.blue
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "part1" => Part::Part1,
            "part2" => Part::Part2,
            _ => Part::Part1, // Default to Part1 if the argument doesn't match
        },
        None => Part::Part1, // Default to Part1 if no argument is provided
    };

    let input_file_path = "./data.txt";
    let bag = BagContent {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum = read_from_file(input_file_path, &bag, part).expect("Failed to read input file");
    println!("id_sum is {}", sum);
}

fn read_from_file(file_path: &str, bag: &BagContent, part: Part) -> Result<u32> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game = read_game_part_1(&line);
                games.push(game);
            },
            Err(err) => return Err(anyhow!(err))
        }
    }
    match part {
        Part::Part1 => {
            let sum = games.iter().filter(|game| game.is_possible(bag)).map(|game| game.id).sum();
            return Ok(sum)
        },
        Part::Part2 => {
            let sum = games.iter().map(|game| game.get_power()).sum();
            return Ok(sum)
        }
    }
}

fn read_game_part_1(description: &str) -> Game {
    let parts: Vec<&str> = description.split(':').collect();
    let id_part = parts[0];
    let sets_part = parts[1];

    let id: u32 = id_part.trim().split_whitespace().last().unwrap().parse().unwrap(); // TODO: remove unwrap and do it properly.
    let sets_descriptions = sets_part.split(';');
    let mut game_sets = Vec::new();

    for set_desc in sets_descriptions {
        let mut game_set = GameSet::new();
        let components = set_desc.split(',').collect::<Vec<&str>>();
        for component in components {
            let details = component.trim().split_whitespace().collect::<Vec<&str>>();
            let quantity = details[0].parse::<u32>().unwrap();
            let color = details[1].to_lowercase();
            game_set.add(&color, quantity);
        }
        game_sets.push(game_set);
    }
    Game { id: id, game_sets: game_sets }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_game_part_1() {
        let a = read_game_part_1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(a.id, 1);
        assert_eq!(a.game_sets.len(), 3);
        assert_eq!(a.game_sets[0].red, 4);
        assert_eq!(a.game_sets[0].blue, 3);
        assert_eq!(a.game_sets[0].green, 0);
        assert_eq!(a.game_sets[1].red, 1);
        assert_eq!(a.game_sets[1].blue, 6);
        assert_eq!(a.game_sets[1].green, 2);
        assert_eq!(a.game_sets[2].red, 0);
        assert_eq!(a.game_sets[2].blue, 0);
        assert_eq!(a.game_sets[2].green, 2);
        assert_eq!(a.fewest_cubes().red, 4);
        assert_eq!(a.fewest_cubes().green, 2);
        assert_eq!(a.fewest_cubes().blue, 6);
        assert_eq!(a.get_power(), 48);

        let b = read_game_part_1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(b.id, 2);
        assert_eq!(b.game_sets.len(), 3);
        assert_eq!(b.game_sets[0].red, 0);
        assert_eq!(b.game_sets[0].blue, 1);
        assert_eq!(b.game_sets[0].green, 2);
        assert_eq!(b.game_sets[1].red, 1);
        assert_eq!(b.game_sets[1].blue, 4);
        assert_eq!(b.game_sets[1].green, 3);
        assert_eq!(b.game_sets[2].red, 0);
        assert_eq!(b.game_sets[2].blue, 1);
        assert_eq!(b.game_sets[2].green, 1);
        assert_eq!(b.fewest_cubes().red, 1);
        assert_eq!(b.fewest_cubes().green, 3);
        assert_eq!(b.fewest_cubes().blue, 4);
        assert_eq!(b.get_power(), 12);

        let c = read_game_part_1("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(c.id, 3);
        assert_eq!(c.game_sets.len(), 3);
        assert_eq!(c.game_sets[0].red, 20);
        assert_eq!(c.game_sets[0].blue, 6);
        assert_eq!(c.game_sets[0].green, 8);
        assert_eq!(c.game_sets[1].red, 4);
        assert_eq!(c.game_sets[1].blue, 5);
        assert_eq!(c.game_sets[1].green, 13);
        assert_eq!(c.game_sets[2].red, 1);
        assert_eq!(c.game_sets[2].blue, 0);
        assert_eq!(c.game_sets[2].green, 5);
        assert_eq!(c.fewest_cubes().red, 20);
        assert_eq!(c.fewest_cubes().green, 13);
        assert_eq!(c.fewest_cubes().blue, 6);
        assert_eq!(c.get_power(), 1560);
        
        let d = read_game_part_1("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(d.id, 4);
        assert_eq!(d.game_sets.len(), 3);
        assert_eq!(d.game_sets[0].red, 3);
        assert_eq!(d.game_sets[0].blue, 6);
        assert_eq!(d.game_sets[0].green, 1);
        assert_eq!(d.game_sets[1].red, 6);
        assert_eq!(d.game_sets[1].blue, 0);
        assert_eq!(d.game_sets[1].green, 3);
        assert_eq!(d.game_sets[2].red, 14);
        assert_eq!(d.game_sets[2].blue, 15);
        assert_eq!(d.game_sets[2].green, 3);
        assert_eq!(d.fewest_cubes().red, 14);
        assert_eq!(d.fewest_cubes().green, 3);
        assert_eq!(d.fewest_cubes().blue, 15);
        assert_eq!(d.get_power(), 630);

        let e = read_game_part_1("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(e.id, 5);
        assert_eq!(e.game_sets.len(), 2);
        assert_eq!(e.game_sets[0].red, 6);
        assert_eq!(e.game_sets[0].blue, 1);
        assert_eq!(e.game_sets[0].green, 3);
        assert_eq!(e.game_sets[1].red, 1);
        assert_eq!(e.game_sets[1].blue, 2);
        assert_eq!(e.game_sets[1].green, 2);
        assert_eq!(e.fewest_cubes().red, 6);
        assert_eq!(e.fewest_cubes().green, 3);
        assert_eq!(e.fewest_cubes().blue, 2);
        assert_eq!(e.get_power(), 36);
    }

     #[test]
    fn test_parts() {
        let bag = BagContent {
            red: 12,
            green: 13,
            blue: 14,
        };
        let part_1 = read_from_file("./part1_test_data.txt", &bag, Part::Part1);
        assert_eq!(part_1.unwrap(), 8);
        let part_2 = read_from_file("./part1_test_data.txt", &bag, Part::Part2);
        assert_eq!(part_2.unwrap(), 2286);
    }
}
