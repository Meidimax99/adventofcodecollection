use std::fs::File;
use std::io::{self, *};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn decode(s: &str) -> usize {
    match s {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!(),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
    let table = vec![vec![4, 8, 3], vec![1, 5, 9], vec![7, 2, 6]];
    let mut sum = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                sum += table[decode(&line[0..1])][decode(&line[2..3])];
            }
        }
    }
    println!("Sum: {}", sum);
}

fn part2() {
    let table = vec![vec![4, 8, 3], vec![1, 5, 9], vec![7, 2, 6]];
    let signtoplay = vec![vec![2, 0, 1], vec![0, 1, 2], vec![1, 2, 0]];
    let mut sum = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let sign = signtoplay[decode(&line[0..1])][decode(&line[2..3])];
                sum += table[decode(&line[0..1])][sign];
            }
        }
    }
    println!("Sum2: {}", sum);
}
