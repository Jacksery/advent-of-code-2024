use std::fs;

fn main() {
    // Read the content of input.txt
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Regular expression to find all valid mul(a,b) patterns
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Initialize the sum
    let mut sum = 0;

    // Iterate over all matches
    for cap in re.captures_iter(&content) {
        // Extract the values of a and b
        let a: u32 = cap[1].parse().unwrap();
        let b: u32 = cap[2].parse().unwrap();

        // Calculate the product and add to the sum
        sum += a * b;
    }

    // Print the final sum
    println!("The sum of all valid mul(a,b) is: {}", sum);
}
