use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn move_forward(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0, pos.1.wrapping_sub(1)),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::West => (pos.0.wrapping_sub(1), pos.1),
        }
    }
}

struct Map {
    map: Vec<Vec<char>>,
    guard_pos: (usize, usize),
    guard_dir: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let (guard_pos, guard_dir) = Self::find_guard(&map);
        Self {
            map,
            guard_pos,
            guard_dir,
            visited: HashSet::new(),
        }
    }

    fn find_guard(map: &Vec<Vec<char>>) -> ((usize, usize), Direction) {
        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    '^' => return ((x, y), Direction::North),
                    'v' => return ((x, y), Direction::South),
                    '<' => return ((x, y), Direction::West),
                    '>' => return ((x, y), Direction::East),
                    _ => {}
                }
            }
        }
        panic!("Guard not found in the map");
    }

    fn move_guard(&mut self) {
        loop {
            self.visited.insert(self.guard_pos);
            self.map[self.guard_pos.1][self.guard_pos.0] = 'X';

            let mut next_pos = self.guard_dir.move_forward(self.guard_pos);

            while self.is_within_bounds(next_pos) && self.map[next_pos.1][next_pos.0] == '#' {
                self.guard_dir = self.guard_dir.turn_right();
                next_pos = self.guard_dir.move_forward(self.guard_pos);
            }

            if !self.is_within_bounds(next_pos) {
                break;
            }

            self.guard_pos = next_pos;

            // Check if the guard has left the map
            if !self.is_within_bounds(self.guard_pos) {
                break;
            }
        }

        println!("Distinct positions visited: {}", self.visited.len());
    }

    fn is_within_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.map[0].len() && pos.1 < self.map.len()
    }

    fn print_map(&self) {
        for row in &self.map {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn main() {
    // read in input.txt as 2d vector
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // create Map instance
    let mut map_instance = Map::new(map);
    map_instance.move_guard();

    // check for command line argument to print the map
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--print-map" {
        map_instance.print_map();
    }
}
