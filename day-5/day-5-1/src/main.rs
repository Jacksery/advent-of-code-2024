use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut lines = input.lines();

    // Read rules
    let mut rules: Vec<(i32, i32)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('|');
        rules.push((
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ));
    }

    // Process instructions
    let sum_middle: i32 = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|instruction| {
            !rules.iter().all(|&(left, right)| {
                match (
                    instruction.iter().position(|&x| x == left),
                    instruction.iter().position(|&x| x == right),
                ) {
                    (Some(l), Some(r)) => l <= r,
                    _ => true,
                }
            })
        })
        .map(|mut instruction| {
            // Sort the instruction based on rules
            instruction.sort_by(|&a, &b| {
                if rules.iter().any(|&(left, right)| a == right && b == left) {
                    std::cmp::Ordering::Greater
                } else if rules.iter().any(|&(left, right)| a == left && b == right) {
                    std::cmp::Ordering::Less
                } else {
                    b.cmp(&a) // Default to descending order for numbers not in rules
                }
            });
            instruction
        })
        .map(|instruction| instruction[instruction.len() / 2])
        .sum();

    println!(
        "Sum of middle page numbers of reordered sequences: {}",
        sum_middle
    );
}
