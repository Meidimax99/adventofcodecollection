use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;

fn main() {
    let path = Path::new("input.txt");

    let file = match File::open(path)  {
        Ok(v) => v,
        Err(_) => {
            println!("Could not open file!");
            return
        },
    };


    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    _ = buf_reader.read_to_string(&mut contents);

    let mut row1: Vec<usize> = vec![];
    let mut row2: Vec<usize> = vec![];


    for line in contents.split("\n") {
        let re = Regex::new(r"\s+").unwrap();
        let split: Vec<&str> = re.split(line).collect();

        //println!("{:?}", split);

        if split.len() == 2 {
            let num1: usize = split[0].parse().unwrap();
            let num2: usize = split[1].parse().unwrap();

            row1.push(num1);
            row2.push(num2);

        }

    }

    assert!(row1.len() == row2.len()); 

    row1.sort();
    row2.sort();

    let mut sum: usize = 0;
    for i in 0..row1.len() {
        let diff: usize;
        diff = row1[i].abs_diff(row2[i]);
        sum += diff;
    }
    print!("Distance: {}\n",sum);

    let mut similarity = 0;
    let mut index2 = 0;
    for i in 0..row1.len() {
        let num1 = row1[i];
        while row2[index2] < num1 {
            index2 += 1;
        }
        let mut count = 0;
        while row2[index2] == num1 {
            index2 += 1;
            count += 1;
        }
        similarity += count * num1;
    }

    print!("Similarity: {}\n", similarity);

}
