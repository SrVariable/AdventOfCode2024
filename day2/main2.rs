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
            let unsafe_index = get_unsafe_index(&nums, nums[0] < nums[1]);
            if unsafe_index < 0 {
                count += 1;
            } else {
                let index: usize = unsafe_index as usize;
                let mut nums_clone = nums.clone();
                nums_clone.remove(index);
                if get_unsafe_index(&nums_clone, nums_clone[0] < nums_clone[1]) == -1 {
                    count += 1;
                } else {
                    nums_clone = nums.clone();
                    nums_clone.remove(0);
                    // Edge case where removing the first element would result in a safe report
                    if get_unsafe_index(&nums_clone, nums_clone[0] < nums_clone[1]) == -1 {
                        count += 1;
                    } else if index + 1 < nums.len() {
                        nums_clone = nums.clone();
                        nums_clone.remove(index + 1);
                        if get_unsafe_index(&nums_clone, nums_clone[0] < nums_clone[1]) == -1 {
                            count += 1;
                        }
                    }
                }
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

fn get_unsafe_index(nums: &Vec<i32>, is_ascending: bool) -> i32 {
    for i in 0..nums.len() - 1 {
        let current = nums[i];
        let next = nums[i + 1];
        let diff = (current - next).abs();
        if is_ascending && current > next || !is_ascending && current < next || diff < 1 || diff > 3
        {
            return i as i32;
        }
    }
    -1
}
