use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;


fn get_input(contents: &String) -> Result<Vec<(usize, usize)>, String> {

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let int_re = Regex::new(r"\d+").unwrap();

    let parsed_input: Vec<(usize, usize)> = re.find_iter(&contents)
        .map(|mat| mat.as_str())
        .map(|str_match| {
            let nums: Vec<usize> = int_re.find_iter(str_match)
                .map(|mat| mat.as_str().parse::<usize>().unwrap())
                .collect();
            if nums.len() == 2 {
                return (nums[0], nums[1]);
            }
            (0,0)
        })
        .collect();

    return Ok(parsed_input);
}

#[derive(PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul((usize,usize)),
    ParseErr
}

fn parse_input(contents: &String) -> Result<Vec<Instruction>, String> {
    
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let int_re = Regex::new(r"\d+").unwrap();

    let parsed_input: Vec<Instruction> = re.find_iter(&contents)
        .map(|mat| mat.as_str())
        .map(|str_match| {
            if str_match == "do()" {
                return Instruction::Do;
            } else if str_match == "don't()" {
                return Instruction::Dont;
            } else  {
                let nums: Vec<usize> = int_re.find_iter(str_match)
                    .map(|mat| mat.as_str().parse::<usize>().unwrap())
                    .collect();
                if nums.len() == 2 {
                    return Instruction::Mul((nums[0], nums[1]));
                }
            }
            Instruction::ParseErr
        })
        .collect();
    return Ok(parsed_input);
}


fn main() {

    //File reading
    let path = Path::new("input.txt");
    
    let file = match File::open(path)  {
        Ok(v) => v,
        Err(_) => {
            println!("{:?}", String::from("Could not open file!"));
            return
        },
    };

    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();

    _ = buf_reader.read_to_string(&mut buf);

    let contents = buf;
    

    //Part 1
    let input = match get_input(&contents) {
        Ok(v)=> v,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };

    let sum = input.into_iter().fold(0, |acc, (a,b)| acc + a*b);

    println!("Sum of Multiplications: {}", sum);


    //Part 2
    let input = match parse_input(&contents) {
        Ok(v)=> v,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };
    let mut enabled = true;
    let sum = input.into_iter()
        .filter( |instr| {
            match instr {
                Instruction::Do => {
                    enabled = true;
                    return false;
                },
                Instruction::Dont => {
                    enabled = false;
                    return false;
                },
                Instruction::Mul(_) => {
                    return enabled;
                },
                Instruction::ParseErr => {
                    return false;
                }
            }
        })
        .fold(0, |acc, instr| {
            if let Instruction::Mul((a,b)) = instr {
                return acc + a*b;
            }
            return acc;
        }
        );

    println!("Sum of Multiplications with control instructions: {}", sum);
   
}
