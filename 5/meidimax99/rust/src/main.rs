use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};


fn parse_input(contents: &String) -> Result<(Vec<(usize, usize)>,Vec<Vec<usize>>), String> {
    
    let mut rules: Vec<(usize,usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];

    return Ok((rules, updates));
}

fn read_file(path: &String) -> Result<String, String> {
    //File reading
    let path = Path::new(path);
        
    let file = match File::open(path)  {
        Ok(v) => v,
        Err(_) => {
            return Err(String::from("Could not open file!"))
        },
    };

    let mut buf_reader = BufReader::new(file);
    let mut buf = String::new();

    _ = buf_reader.read_to_string(&mut buf);

    let contents = buf;

    return Ok(contents)
}


fn main() {

    let path = String::from("input.txt");

    let (rules, updates) = parse_input(&read_file(&path).unwrap()).unwrap();

    //Part 1
    

    println!("Sum of Middle Page numbers of correctly ordered Updates: {}", sum);
   
}
