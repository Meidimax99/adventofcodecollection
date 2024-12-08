use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

fn parse_input(contents: &String) -> Result<Vec<Vec<char>>, String> {
    
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        matrix.push(line.chars().collect());
    }

    return Ok(matrix);
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
#[derive(EnumIter, Debug)]
enum Direction{
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn next_idx(cur_idx: (usize, usize), dir: &Direction, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
    let mut x = cur_idx.0 as i32;
    let mut y = cur_idx.1 as i32;

    let max_x = max_x as i32;
    let max_y = max_y as i32;

    match dir {
        Direction::North => {
            if y - 1 >= 0 {
                y -= 1;
            } else {
                return None;
            }
        },
        Direction::NorthEast => {
            if y-1 >= 0 && x + 1 < max_x{
                y -= 1;
                x += 1;
            } else {
                return None;
            }
        },
        Direction::East => {
            if x+1 < max_x {
                x  += 1
            } else {
                return None;
            }
        },
        Direction::SouthEast => {
            if x+1 < max_x && y + 1 < max_y{
                x += 1;
                y += 1;
            } else {
                return None;
            }
        },
        Direction::South => {
            if y + 1 < max_y{
                y += 1
            } else {
                return None;
            }
        },
        Direction::SouthWest => {
            if x - 1 >= 0 && y + 1 < max_y{
                x -= 1;
                y += 1;
            } else {
                return None;
            }
        },
        Direction::West => {
            if x - 1 >= 0{
                x -= 1;
            } else {
                return None;
            }
        },
        Direction::NorthWest => {
            if x - 1 >= 0 && y - 1 >= 0{
                x -= 1;
                y -= 1;
            } else {
                return None;
            }
        },
    }
    println!("Next {:?} idx after [{},{}] is [{},{}]",dir, cur_idx.0, cur_idx.1, x, y);
    return Some((x as usize ,y as usize))
    
}

fn contains_pat(pattern: &String,  matrix: &Vec<Vec<char>>, idx: (usize, usize), dir: Direction )  -> bool {
    let pat_len = pattern.len();
    let pat_arr: Vec<char> = pattern.chars().collect();

    let mat_x = matrix[0].len();
    let mat_y = matrix.len();

    //TODO Check for the first field can be removed
    let (mut x,mut y) = idx;
    for i in 0..pat_len {
        println!("{} - {}[{},{}]", pat_arr[i], matrix[y][x], x,y);
        if pat_arr[i] == matrix[y][x] {
            let new_idx = next_idx((x,y), &dir, mat_x, mat_y);
            if new_idx.is_none() {
                return false;
            }
            (x, y) = new_idx.unwrap();
            continue;
        }
        return false;
    }
    return true;
}
//idx: x, y
fn check_field(pattern: &String, matrix: &Vec<Vec<char>>, idx: (usize, usize)) -> usize{

    let mut sum = 0;

    if pattern.chars().collect::<Vec<char>>()[0] == matrix[idx.1][idx.0] {
        for dir in Direction::iter() {
            println!("Checking direction {:?}", dir);
            if contains_pat(pattern, matrix, idx, dir) {
                sum += 1;
            }
        }
    }

    sum

}

fn find_pat(pattern: &String, matrix: Vec<Vec<char>>)  -> Result<usize, String>  {

    let mat_x = matrix[0].len();
    let mat_y = matrix.len();

    let mut sum = 0;
    for y in 0..mat_y {
        for x in 0..mat_x {
            //println!("Checking [{},{}]\n", x,y);
            sum += check_field(pattern, &matrix, (x,y));
        }
    }
    return Ok(sum)
}

/*

  x =>
y   OOOOOOOOOO
||  OOOOOOOOOO
\/  OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
    OOOOOOOOOO
*/
fn main() {

    let path = String::from("test_input.txt");
    let pattern = String::from("XMAS");

    let matrix = parse_input(&read_file(&path).unwrap()).unwrap();

    //Part 1
    
    let num = find_pat(&pattern, matrix);
    println!("Number of Matches: {}", num.unwrap());


    //Part 2
    

    //println!("Sum of Multiplications with control instructions: {}", sum);
   
}
