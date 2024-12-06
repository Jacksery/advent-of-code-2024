use fixedbitset::FixedBitSet;
use rayon::prelude::*;

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

    #[inline(always)]
    fn move_forward(self, pos: usize, width: usize) -> Option<usize> {
        match self {
            Direction::North => pos.checked_sub(width),
            Direction::South => Some(pos + width),
            Direction::East => {
                if (pos + 1) % width != 0 {
                    Some(pos + 1)
                } else {
                    None
                }
            }
            Direction::West => {
                if pos % width != 0 {
                    Some(pos - 1)
                } else {
                    None
                }
            }
        }
    }
}

struct Map {
    map: Vec<char>,
    width: usize,
    guard_pos: usize,
    guard_dir: Direction,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let mut map = Vec::with_capacity(width * input.lines().count());
        let mut guard_pos = 0;
        let mut guard_dir = Direction::North;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = y * width + x;
                match c {
                    '^' => {
                        guard_pos = pos;
                        guard_dir = Direction::North;
                        map.push('.');
                    }
                    'v' => {
                        guard_pos = pos;
                        guard_dir = Direction::South;
                        map.push('.');
                    }
                    '<' => {
                        guard_pos = pos;
                        guard_dir = Direction::West;
                        map.push('.');
                    }
                    '>' => {
                        guard_pos = pos;
                        guard_dir = Direction::East;
                        map.push('.');
                    }
                    c => map.push(c),
                }
            }
        }

        Self {
            map,
            width,
            guard_pos,
            guard_dir,
        }
    }

    #[inline(always)]
    fn guard_gets_stuck(&self, obstruction_pos: usize) -> bool {
        let mut visited = FixedBitSet::with_capacity(self.map.len() * 4);
        let mut pos = self.guard_pos;
        let mut dir = self.guard_dir;
        let mut state = 0usize;

        while state < self.map.len() * 4 {
            let visit_idx = pos * 4 + dir as usize;
            if visited.contains(visit_idx) {
                return true;
            }
            visited.insert(visit_idx);
            state += 1;

            let mut next_pos = match dir.move_forward(pos, self.width) {
                Some(p) if p < self.map.len() => p,
                _ => return false,
            };

            if next_pos == obstruction_pos || self.map[next_pos] == '#' {
                let initial_dir = dir;
                loop {
                    dir = dir.turn_right();
                    if dir == initial_dir {
                        return true;
                    }
                    if let Some(p) = dir.move_forward(pos, self.width) {
                        if p < self.map.len() && p != obstruction_pos && self.map[p] != '#' {
                            next_pos = p;
                            break;
                        }
                    }
                }
            }
            pos = next_pos;
        }
        true
    }

    fn get_candidate_positions(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .filter(|&(i, &c)| c == '.' && i != self.guard_pos)
            .map(|(i, _)| i)
            .collect()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&input);
    let candidates = map.get_candidate_positions();

    let chunk_size = (candidates.len() / rayon::current_num_threads()).max(1);
    let count = candidates
        .par_chunks(chunk_size)
        .map(|chunk| {
            chunk
                .iter()
                .filter(|&&pos| map.guard_gets_stuck(pos))
                .count()
        })
        .sum::<usize>();

    println!("Number of obstruction positions causing a loop: {}", count);
}
