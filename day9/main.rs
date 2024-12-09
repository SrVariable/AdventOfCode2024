use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 {
        filename = &args[1];
    }
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            part_one(&line);
            part_two(&line);
        }
    }
}

fn part_one(line: &str) {
    let mut sequence = get_sequence(&line);
    let mut total_sum = 0;
    let mut end = sequence.len() - 1;
    for i in 0..sequence.len() {
        if i >= end {
            break;
        }
        if sequence[i] == usize::MAX {
            while sequence[end] == usize::MAX {
                end -= 1;
            }
            sequence[i] = sequence[end];
            sequence[end] = usize::MAX;
        }
        total_sum += i * sequence[i];
    }
    println!("{}", total_sum);
}

fn get_sequence(line: &str) -> Vec<usize> {
    let mut sequence: Vec<usize> = vec![];
    let mut value = 0;
    for i in 0..line.len() {
        for _ in 0..line.as_bytes()[i] - b'0' {
            if i % 2 == 0 {
                sequence.push(value);
            } else {
                sequence.push(usize::MAX);
            }
        }
        if i % 2 == 0 {
            value += 1;
        }
    }
    sequence
}

fn part_two(line: &str) {
    let mut sequence = get_sequence(&line);
    let mut end = sequence.len() - 1;
    while end > 0 {
        while sequence[end] == usize::MAX {
            end -= 1;
        }
        let value = sequence[end];
        let amount = get_amount_of_values(&sequence, end);
        let mut i = 0;
        while i < end {
            if sequence[i] == usize::MAX {
                let free_space = get_free_space(&sequence, i);
                if amount <= free_space {
                    for j in 0..amount {
                        sequence[i + j] = value;
                        sequence[end - j] = usize::MAX;
                    }
                    break;
                }
            }
            i += 1;
        }
        if amount > end {
            break;
        }
        end -= amount;
    }
    let mut total_sum = 0;
    for i in 0..sequence.len() {
        if sequence[i] != usize::MAX {
            total_sum += i * sequence[i];
        }
    }
    println!("{}", total_sum);
}

fn get_amount_of_values(sequence: &Vec<usize>, end: usize) -> usize {
    let mut amount = 0;
    let value = sequence[end];
    let mut j = end;
    while j > 0 {
        if sequence[j] != value {
            break;
        }
        amount += 1;
        j -= 1;
    }
    if sequence[j] == value {
        amount += 1;
    }
    amount
}

fn get_free_space(sequence: &Vec<usize>, start: usize) -> usize {
    let mut count = 0;
    for i in start..sequence.len() {
        if sequence[i] != usize::MAX {
            break;
        }
        count += 1;
    }
    count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
