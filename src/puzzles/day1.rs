use crate::utils;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    println!("Solving day 1...");
    let file_path = utils::get_input_file("1");
    match utils::read_columns(&file_path) {
        Ok((col1, col2)) => {
            assert_eq!(col1.len(), col2.len());
            let mut sorted_col1 = col1.clone();
            let mut sorted_col2 = col2.clone();
            sorted_col1.sort();
            sorted_col2.sort();

            let difference = sorted_col1
                .iter()
                .zip(sorted_col2.iter())
                .map(|(a, b)| a - b);
            let distance = difference.map(|x| x.abs());
            let total_distance: i32 = distance.sum();
            println!("    Day 1, Part 1 Solution: {}", total_distance);

            let unique: HashSet<i32> = col1.into_iter().collect();
            let mut counts: HashMap<i32, i32> = unique.iter().map(|&key| (key, 0)).collect();

            for &num in &col2 {
                if unique.contains(&num) {
                    *counts.entry(num).or_insert(0) += 1;
                }
            }

            let dot_product: i32 = counts.iter().map(|(key, value)| key * value).sum();

            println!("    Day 1 Part 2 Solution: {}", dot_product);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
