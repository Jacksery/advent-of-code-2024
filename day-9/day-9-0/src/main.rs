use std::fs::File;
use std::io::Read;

fn main() {
    // Read the input from "input.txt"
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    file.read_to_string(&mut input)
        .expect("Failed to read from input.txt");
    let disk_map = input.trim();

    // Parse the disk map into segments: (length, is_file, file_id)
    let mut segments = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for c in disk_map.chars() {
        let length = match c.to_digit(10) {
            Some(n) => n as usize,
            None => continue, // Skip invalid characters silently
        };
        segments.push((length, is_file, if is_file { Some(file_id) } else { None }));
        if is_file && length > 0 {
            file_id += 1;
        }
        is_file = !is_file;
    }

    // Build the initial disk layout
    let mut disk = Vec::new();
    for (length, is_file, fid_opt) in &segments {
        if *is_file {
            if let Some(fid) = fid_opt {
                for _ in 0..*length {
                    disk.push(Some(*fid));
                }
            }
        } else {
            for _ in 0..*length {
                disk.push(None);
            }
        }
    }

    // Simulate moving blocks: Move one block at a time from the end to the first available gap
    loop {
        // Find the leftmost free space (gap)
        if let Some(gap_pos) = disk.iter().position(|&b| b.is_none()) {
            // Find the rightmost file block after this gap
            let file_pos_opt = disk
                .iter()
                .enumerate()
                .filter(|&(pos, &b)| pos > gap_pos && b.is_some())
                .map(|(pos, _)| pos)
                .last();

            if let Some(file_pos) = file_pos_opt {
                let fid = disk[file_pos].unwrap();

                // Perform the move without printing
                disk[gap_pos] = Some(fid);
                disk[file_pos] = None;
            } else {
                break; // No more movable file blocks
            }
        } else {
            break; // No gaps remaining
        }
    }

    // Calculate checksum: sum(position * file_id) for each file block
    let checksum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|fid| pos * fid))
        .sum();

    // Print the final checksum
    println!("Final Checksum: {}", checksum);
}
