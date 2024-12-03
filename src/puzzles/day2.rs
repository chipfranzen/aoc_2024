use crate::utils;

pub fn parse_line_to_vec(line: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    line.split_whitespace().map(|s| s.parse::<i32>()).collect()
}

pub fn monotonic(nums: &Vec<i32>) -> bool {
    assert!(nums.len() > 2);
    match nums[0] > nums[1] {
        true => nums.windows(2).all(|pair| pair[0] > pair[1]),

        false => nums.windows(2).all(|pair| pair[0] < pair[1]),
    }
}

pub fn safe_difference(nums: &Vec<i32>) -> bool {
    nums.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        diff >= 1 && diff <= 3
    })
}

pub fn is_safe(nums: &Vec<i32>) -> bool {
    monotonic(nums) && safe_difference(nums)
}

pub fn damped_safe(nums: &Vec<i32>) -> bool {
    if is_safe(nums) {
        return true;
    }

    for i in 0..nums.len() {
        let mut damped = nums.clone();
        damped.remove(i);
        if is_safe(&damped) {
            return true;
        }
    }

    false
}

pub fn solve() {
    println!("Solving day 2...");
    let input_file = utils::get_input_file("2");
    match utils::read_lines(&input_file) {
        Ok(lines) => {
            let parsed = lines.iter().map(|s| parse_line_to_vec(s).unwrap());
            let total_safe_reports: i32 = parsed.clone().map(|v| is_safe(&v) as i32).sum();
            println!("    Day 2, Part 1 Solution: {}", total_safe_reports);

            let total_damped_safe_reports: i32 =
                parsed.clone().map(|v| damped_safe(&v) as i32).sum();
            println!("    Day 2, Part 1 Solution: {}", total_damped_safe_reports);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
