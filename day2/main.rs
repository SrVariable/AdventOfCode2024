use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut count = 0;
        for line in lines.flatten() {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            if safe_check(&nums, nums[0] < nums[1]) {
                count += 1;
            }
        }
        println!("{}", count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn safe_check(nums: &Vec<i32>, is_ascending: bool) -> bool {
    for i in 0..nums.len() - 1 {
        let current = nums[i];
        let next = nums[i + 1];
        let diff = (current - next).abs();
        if is_ascending && current > next || !is_ascending && current < next || diff < 1 || diff > 3
        {
            return false;
        }
    }
    true
}
