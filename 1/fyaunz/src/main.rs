use std::fs::File;
use std::io::{self, *};
use std::path::Path;

const TOP_ELVES: usize = 3;

fn main() {
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
    let mut current = 0;
    let mut max = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let cal: i32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => -1,
                };
                if cal != -1 {
                    current += cal;
                } else {
                    if current > max {
                        max = current;
                    }
                    current = 0;
                }
            }
        }
        println!("task1 max: {}", max);
    }
}

fn part2() {
    let mut current = 0;
    let mut max = Vec::new();
    let mut result = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let cal: i32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => -1,
                };
                if cal != -1 {
                    current += cal;
                } else {
                    max.push(current);
                    current = 0;
                }
            }
        }
        max.sort();
        for i in max.len() - TOP_ELVES..max.len() {
            result += max[i];
        }
        println!("task 2: {}", result);
    }
}
