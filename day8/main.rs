use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 {
        filename = &args[1];
    }
    if let Ok(lines) = read_lines(filename) {
        let mut pairs = HashMap::<char, Vec<Point>>::new();
        let mut rows = 0;
        let mut cols = 0;
        for line in lines.flatten() {
            cols = line.len();
            for i in 0..cols {
                let key = line.as_bytes()[i] as char;
                if key != '.' {
                    pairs.entry(key).or_insert(vec![]).push(Point {
                        x: rows,
                        y: i as i32,
                    });
                }
            }
            rows += 1;
        }
        let cols = cols as i32;
        part_one(&pairs, rows, cols);
        part_two(&pairs, rows, cols);
    }
}

fn part_one(pairs: &HashMap<char, Vec<Point>>, rows: i32, cols: i32) {
    let mut set = HashSet::<Point>::new();
    for (_, points) in pairs {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[i].x - points[j].x;
                let dy = points[i].y - points[j].y;
                let p1 = Point {
                    x: points[i].x + dx,
                    y: points[i].y + dy,
                };
                let p2 = Point {
                    x: points[j].x - dx,
                    y: points[j].y - dy,
                };
                if p1.x >= 0 && p1.x < rows && p1.y >= 0 && p1.y < cols {
                    set.insert(p1);
                }
                if p2.x >= 0 && p2.x < rows && p2.y >= 0 && p2.y < cols {
                    set.insert(p2);
                }
            }
        }
    }
    println!("{}", set.len());
}

fn part_two(pairs: &HashMap<char, Vec<Point>>, rows: i32, cols: i32) {
    let mut set = HashSet::<Point>::new();
    for (_, points) in pairs {
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[i].x - points[j].x;
                let dy = points[i].y - points[j].y;
                let mut p1 = Point {
                    x: points[i].x + dx,
                    y: points[i].y + dy,
                };
                let mut p2 = Point {
                    x: points[j].x - dx,
                    y: points[j].y - dy,
                };
                while p1.x >= 0 && p1.x < rows && p1.y >= 0 && p1.y < cols {
                    set.insert(p1.clone());
                    p1 = Point {
                        x: p1.x + dx,
                        y: p1.y + dy,
                    };
                }
                while p2.x >= 0 && p2.x < rows && p2.y >= 0 && p2.y < cols {
                    set.insert(p2.clone());
                    p2 = Point {
                        x: p2.x - dx,
                        y: p2.y - dy,
                    };
                }
                set.insert(points[i].clone());
                set.insert(points[j].clone());
            }
        }
    }
    println!("{}", set.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
