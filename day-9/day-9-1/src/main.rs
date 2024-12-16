use std::fs::File;
use std::io::Read;

fn main() {
    // Read input
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    file.read_to_string(&mut input)
        .expect("Failed to read input.txt");

    // Parse input and build disk layout
    let mut disk = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for c in input.trim().chars() {
        if let Some(length) = c.to_digit(10) {
            let length = length as usize;
            if is_file && length > 0 {
                for _ in 0..length {
                    disk.push(Some(file_id));
                }
                file_id += 1;
            } else {
                for _ in 0..length {
                    disk.push(None);
                }
            }
            is_file = !is_file;
        }
    }

    // Collect file information
    let mut files = Vec::new();
    let mut current_file = None;
    let mut start_pos = 0;

    for (pos, &cell) in disk.iter().enumerate() {
        match (current_file, cell) {
            (None, Some(id)) => {
                current_file = Some((id, pos)); // Start new file
            }
            (Some((id, start)), Some(curr_id)) if id != curr_id => {
                files.push((id, start, pos - start)); // End previous file
                current_file = Some((curr_id, pos)); // Start new file
            }
            (Some((id, start)), None) => {
                files.push((id, start, pos - start)); // End file at gap
                current_file = None;
            }
            _ => {}
        }
    }

    // Add final file if exists
    if let Some((id, start)) = current_file {
        files.push((id, start, disk.len() - start));
    }

    // Sort files by ID descending
    files.sort_by_key(|&(id, _, _)| std::cmp::Reverse(id));

    // Process each file
    for (file_id, start, length) in files {
        // Find leftmost free space that fits
        let mut consecutive_free = 0;
        let mut free_start = None;

        for (pos, &cell) in disk[..start].iter().enumerate() {
            if cell.is_none() {
                if free_start.is_none() {
                    free_start = Some(pos);
                }
                consecutive_free += 1;
                if consecutive_free == length {
                    // Found suitable space - move file
                    if let Some(free_pos) = free_start {
                        // Clear original position
                        for i in start..start + length {
                            disk[i] = None;
                        }
                        // Place in new position
                        for i in free_pos..free_pos + length {
                            disk[i] = Some(file_id);
                        }
                    }
                    break;
                }
            } else {
                consecutive_free = 0;
                free_start = None;
            }
        }
    }

    // Calculate checksum
    let checksum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(pos, &cell)| cell.map(|id| pos * id))
        .sum();

    println!("Final Checksum: {}", checksum);
}
