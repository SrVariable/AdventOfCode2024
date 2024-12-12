use std::collections::HashMap;
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
    let mut sequence: HashMap<usize, usize> = line
        .split_whitespace()
        .map(|s| (s.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<usize, usize>>();
    for _ in 0..25 {
        sequence = get_new_sequence(&sequence);
    }
    let mut total_sum = 0;
    for (_, value) in sequence {
        total_sum += value;
    }
    println!("{}", total_sum);
}

fn part_two(line: &str) {
    let mut sequence: HashMap<usize, usize> = line
        .split_whitespace()
        .map(|s| (s.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<usize, usize>>();
    for _ in 0..75 {
        sequence = get_new_sequence(&sequence);
    }
    let mut total_sum = 0;
    for (_, value) in sequence {
        total_sum += value;
    }
    println!("{}", total_sum);
}

fn get_new_sequence(sequence: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_sequence = HashMap::new();
    for (&key, &value) in sequence {
        match key {
            0 => *new_sequence.entry(1).or_default() += value,
            key if key.to_string().len() % 2 == 0 => {
                let s = key.to_string();
                *new_sequence
                    .entry(s[..s.len() / 2].parse::<usize>().unwrap())
                    .or_default() += value;
                *new_sequence
                    .entry(s[s.len() / 2..].parse::<usize>().unwrap())
                    .or_default() += value;
            }
            _ => *new_sequence.entry(key * 2024).or_default() += value,
        }
    }
    new_sequence
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
