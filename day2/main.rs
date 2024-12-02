use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut count = 0;
        for line in lines.flatten() {
            let mut is_safe = true;
            let nums: Vec<&str> = line.split_whitespace().collect();
            let is_incr = nums[0].parse::<i32>().unwrap() < nums[1].parse::<i32>().unwrap();
            for i in 0..nums.len() - 1 {
                let current = nums[i].parse::<i32>().unwrap();
                let next = nums[i + 1].parse::<i32>().unwrap();
                let diff =  current - next;
                if is_incr && current > next || !is_incr && current < next || diff.abs() < 1 || diff.abs() > 3 {
                    is_safe = false;
                    break;
                }
            }
            if is_safe {
                count += 1;
            }
        }
        println!("{}", count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
