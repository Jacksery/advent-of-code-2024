use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // Read the input file and collect towers by frequency
    let mut towers_by_freq = HashMap::new();
    let (mut width, mut height) = (0, 0);
    for (y, line) in fs::read_to_string("input.txt")
        .expect("Failed to read input.txt")
        .lines()
        .enumerate()
    {
        height = height.max(y as i32 + 1);
        width = width.max(line.len() as i32);
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                towers_by_freq
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    // Calculate antinodes
    let mut antinodes = HashSet::new();
    for positions in towers_by_freq.values() {
        if positions.len() >= 2 {
            // Add antennas themselves as antinodes
            antinodes.extend(positions);

            for (i, &(x1, y1)) in positions.iter().enumerate() {
                for &(x2, y2) in &positions[i + 1..] {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let gcd_val = gcd(dx.abs(), dy.abs());
                    let (step_x, step_y) = (dx / gcd_val, dy / gcd_val);

                    // Step in positive direction
                    let mut x = x1 + step_x;
                    let mut y = y1 + step_y;
                    while (0..width).contains(&x) && (0..height).contains(&y) {
                        antinodes.insert((x, y));
                        x += step_x;
                        y += step_y;
                    }

                    // Step in negative direction
                    x = x1 - step_x;
                    y = y1 - step_y;
                    while (0..width).contains(&x) && (0..height).contains(&y) {
                        antinodes.insert((x, y));
                        x -= step_x;
                        y -= step_y;
                    }
                }
            }
        }
    }

    // Output the result
    println!("Total unique antinode locations: {}", antinodes.len());
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
