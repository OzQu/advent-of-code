use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize)
}

#[derive(Debug, Default)]
struct Location {
    horizontal: usize,
    depth: usize,
    aim: usize
}

impl Location {
    fn move_to(&mut self, directions: &Vec<Direction>) -> () {
        for direction in directions {
            match direction {
                Direction::Forward(units) => self.horizontal = self.horizontal + units,
                Direction::Down(units) => self.depth = self.depth + units,
                Direction::Up(units) => self.depth = self.depth - units
            }
        }
    }

    fn aim_and_move(&mut self, directions: &Vec<Direction>) -> () {
        for direction in directions {
            match direction {
                Direction::Forward(units) => {
                    self.horizontal = self.horizontal + units;
                    self.depth = self.depth + (self.aim * units);
                },
                Direction::Down(units) => self.aim = self.aim + units,
                Direction::Up(units) => self.aim = self.aim - units
            }
        }

    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let direction = split.next();
        let units = split.next()
            .unwrap_or("0")
            .parse::<usize>();
        let pair = (direction, units);
        match pair {
            (Some("forward"), Ok(units)) => Ok(Direction::Forward(units)),
            (Some("down"), Ok(units)) => Ok(Direction::Down(units)),
            (Some("up"), Ok(units)) => Ok(Direction::Up(units)),
            _ => Err(())
        }
    }
    
}

fn load_from_file(file_path: &str) -> Vec<Direction> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let directions: Vec<Direction> = reader
        .lines()
        .map(|line| line.unwrap().parse::<Direction>().unwrap())
        .collect();

    directions
}

fn main() {
    const FILENAME: &str = "day2_input.txt";
    let directions = load_from_file(FILENAME);
    println!("{:?}", directions);
    let mut location = Location::default();
    println!("original location: {:?}", location);
    location.move_to(&directions);
    println!("moved location: {:?}", location);
    println!("multiply coordinates: {}", location.depth * location.horizontal);

    let mut location2 = Location::default();
    location2.aim_and_move(&directions);
    println!("moved another location: {:?}", location2);
    println!("multiply coordinates: {}", location2.depth * location2.horizontal);

}
