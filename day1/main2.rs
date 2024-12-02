use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for line in lines.flatten() {
            let nums: Vec<&str> = line.split_whitespace().collect();
            left.push(nums[0].parse().unwrap());
            right.push(nums[1].parse().unwrap());
        }
        let mut total_sum = 0;
        for i in 0..left.len() {
            total_sum += left[i] * (right.iter().filter(|&&x| x == left[i]).count() as i32);
        }
        println!("{}", total_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
