fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let mut total_sum = 0;

    for line in input.lines() {
        let mut partitions = line.split_whitespace();
        let result_str = partitions.next().unwrap_or("");
        let result_value = match result_str[..result_str.len() - 1].parse::<i64>() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error parsing result: {}", e);
                continue;
            }
        };

        let members: Result<Vec<i64>, _> = partitions.map(|p| p.parse()).collect();
        let members = match members {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error parsing members: {}", e);
                continue;
            }
        };

        if let Some(valid_result) = brute_force_all_solutions(&members, result_value) {
            total_sum += valid_result;
        }
    }

    println!("Total sum of valid results: {}", total_sum);
}

fn brute_force_all_solutions(input: &[i64], expected_result: i64) -> Option<i64> {
    let n = input.len();
    let num_operations = 3_i64.pow((n - 1) as u32); // 3 operations per position

    for i in 0..num_operations {
        let mut result = input[0];
        let mut operations = Vec::new();
        let mut temp = i;

        for j in 0..(n - 1) {
            let op = temp % 3;
            temp /= 3;

            match op {
                0 => {
                    // Addition
                    result += input[j + 1];
                    operations.push('+');
                }
                1 => {
                    // Multiplication
                    match result.checked_mul(input[j + 1]) {
                        Some(res) => result = res,
                        None => break, // Overflow occurred
                    }
                    operations.push('*');
                }
                2 => {
                    // Concatenation
                    let concat = format!("{}{}", result, input[j + 1]);
                    match concat.parse::<i64>() {
                        Ok(val) => result = val,
                        Err(_) => break, // Number too large
                    }
                    operations.push('|');
                }
                _ => unreachable!(),
            }
        }

        if result == expected_result {
            return Some(expected_result);
        }
    }

    None
}
