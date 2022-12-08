use std::fs::File;
use std::io::{self, *};
use std::path::Path;

fn main() {
    part1();
}

fn match_char(char: &str) -> i32 {
    return match char {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "D" => 4,
        "E" => 5,
        "F" => 6,
        "G" => 7,
        "H" => 8,
        "I" => 9,
        "J" => 10,
        "K" => 11,
        "L" => 12,
        "M" => 13,
        "N" => 14,
        "O" => 15,
        "P" => 16,
        "Q" => 17,
        "R" => 18,
        "S" => 19,
        "T" => 20,
        "U" => 21,
        "V" => 22,
        "W" => 23,
        "X" => 24,
        "Y" => 25,
        "Z" => 26,
        _ => panic!("Something strange is afoot"),
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn char_to_priority(char: &str) -> i32 {
    let mut prio = 0;
    if char >= "A" && char <= "Z" {
        prio += 26;
    }
    let char = char.to_ascii_uppercase();
    let char = char.as_str();

    prio += match_char(char);
    return prio;
}

fn get_prop(line: String) -> i32 {
    let len = line.len();
    for x in 0..(len / 2) {
        for y in (len / 2)..len {
            if &line[x..x + 1] == &line[y..y + 1] {
                return char_to_priority(&line[x..x + 1]);
            }
        }
    }
    return 0;
}

fn part1() {
    let mut sum = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                sum += get_prop(line);
            }
        }
    }
    println!("Sum: {}", sum);
}
