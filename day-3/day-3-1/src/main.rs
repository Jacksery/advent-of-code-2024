use regex::Regex;
use std::fs;

fn calculate_sum(content: &str) -> u32 {
    // Combined regex to match do(), undo(), don't(), and mul(a,b)
    let re = Regex::new(r"do\(\)|undo\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;

    // Iterate over all matches in order
    for cap in re.captures_iter(content) {
        let matched = cap.get(0).map_or("", |m| m.as_str());

        if matched == "do()" || matched == "undo()" {
            mul_enabled = true;
        } else if matched == "don't()" {
            mul_enabled = false;
        } else if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
            if mul_enabled {
                let a_num: u32 = a.as_str().parse().unwrap();
                let b_num: u32 = b.as_str().parse().unwrap();
                sum += a_num * b_num;
            }
        }
    }

    sum
}

fn main() {
    // Read the content of input.txt
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let sum = calculate_sum(&content);

    // Print the final sum
    println!("The sum of all valid mul(a,b) is: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let content = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(calculate_sum(content), 48);
    }
}
