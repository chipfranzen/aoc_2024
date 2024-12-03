use std::fs::File;
use std::io::{self, BufRead};

pub fn read_columns(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Read each line
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        // Parse the two integers from each line
        if let (Some(val1), Some(val2)) = (parts.next(), parts.next()) {
            column1.push(
                val1.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            );
            column2.push(
                val2.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            );
        }
    }

    Ok((column1, column2))
}

pub fn get_input_file(day: &str) -> String {
    format!("{}/data/day{}_input.txt", env!("CARGO_MANIFEST_DIR"), day)
}

pub fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    reader.lines().collect()
}
