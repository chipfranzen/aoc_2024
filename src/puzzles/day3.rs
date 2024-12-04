use crate::utils;
use regex::Regex;
use std::fs;

pub fn mulsum(content: &str) -> i32 {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    re.captures_iter(content)
        .map(|m| {
            let a: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

fn filter_by_keyword(input: &str) -> String {
    let mut result = String::new();
    for token in input.split("do()").skip(1) {
        if let Some((content, _)) = token.split_once("don't()") {
            result.push_str(content.trim());
            result.push(' ');
        } else {
            result.push_str(token.trim());
            result.push(' ');
        }
    }

    result.trim_end().to_string()
}

pub fn solve() {
    println!("Solving day 3...");
    let input_file = utils::get_input_file("3");

    match fs::read_to_string(input_file) {
        Ok(s) => {
            let part_1_solution = mulsum(&s);
            println!("    Day 3, Part 1 Solution: {}", part_1_solution);

            let filtered_content = filter_by_keyword(&s);
            let part_2_solution: i32 = mulsum(&filtered_content);
            println!("    Day 3, Part 2 Solution: {}", part_2_solution);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
