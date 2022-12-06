use std::fs::File;
use std::io::{*, self};
use std::path::Path;

fn main() {
    let mut current = 0;
    let mut max = 0;
    if let Ok(lines) = read_lines("input.txt"){
        for line in lines {
            if let Ok(line) = line{
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
        println!("MAX: {}", max);
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}