use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use geo::{ Point};
use std::collections::HashMap;
use plotters::prelude::*;

static SCALE: usize = 10;

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

fn draw_stuff(x: usize, y:usize, points: &HashMap<char, Vec<Point>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", ((x * SCALE) as u32, (y * SCALE )as u32)).into_drawing_area();
    root.fill(&WHITE)?;


    for (_, value) in points.iter() {
        for point in value.iter() {
            if SCALE > 1 {
                root.draw(&Circle::new(
                    ((point.x() as usize * SCALE) as i32, (point.y() as usize * SCALE) as i32),
                    1 as i32,        
                    BLUE.stroke_width(1),
                ))?;
            } else {
                root.draw(&Pixel::new(
                    (point.x() as i32, point.y() as i32),
                    BLUE.stroke_width(1),
                ))?;
            }
            
        }
    }

    // root.draw(&PathElement::new(
    //     vec![(100, 300), (300, 500)], // Start and end points
    //     &BLACK,
    // ))?;

    // Save the image
    root.present()?;
    println!("Drawing saved to output.png");
    Ok(())
}

fn parse_map(matrix: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>>{
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let key =  matrix[y][x];
            if key != '.' {
                if !map.contains_key(&key) {
                    map.insert(key, vec![]);
                }
                let vect = map.get_mut(&key).unwrap();
                vect.push(Point::new(x  as f64, y  as f64));
            }
        }
    }
    return map;
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let path = String::from("input.txt");

    let matrix = parse_input(&read_file(&path).unwrap()).unwrap();

    //Part 1
    let sum = 0;
    // create map Symbol -> Points[] by parsing the matrix
    let points = parse_map(&matrix);
    draw_stuff(matrix[0].len(), matrix.len(), &points)?;
    //Create Pair-Wise Lines between any two points belonging to a frequency
    //Draw a two circles for each line, one for each node defining the line
        //Radius is the distance between the nodes
        //Calculate intersection between the line and the circle
            //One intersection is the other node, the other one is the antinode

    //Add antinodes to an additional list or hash entry or whatever

    //Count antinodes

    println!("Number of antinodes: {}", sum);
   
   Ok(())
}


// fn main() 


// use geo::{Line, Point};

// fn main() {
//     let line1 = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
//     let line2 = Line::new(Point::new(0.0, 4.0), Point::new(4.0, 0.0));

//     if let Some(intersection) = line1.intersect(&line2) {
//         println!("Intersection: {:?}", intersection);
//     } else {
//         println!("No intersection.");
//     }
// }
