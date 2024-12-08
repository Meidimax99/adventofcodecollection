use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use geo::{ Line, Point};
use std::collections::HashMap;
use plotters::prelude::*;

static SCALE: i32 = 10;

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

fn draw_stuff(x: usize, y:usize, points: &HashMap<char, Vec<Point>>, lines: &HashMap<char, Vec<Line>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", ((x as i32 * SCALE) as u32, (y as i32 * SCALE )as u32)).into_drawing_area();
    root.fill(&WHITE)?;


    for (_, value) in points.iter() {
        for point in value.iter() {
            if SCALE > 1 {
                root.draw(&Circle::new(
                    ((point.x() as i32 * SCALE) as i32, (point.y() as i32 * SCALE) as i32),
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

    for (_, vec) in lines.iter() {
        for line in vec.iter() {
            let start = line.start;
            let end = line.end;
            root.draw(&PathElement::new(
                vec![(start.x as i32 * SCALE, start.y as i32 * SCALE), (end.x as i32  * SCALE, end.y as i32  * SCALE)], // Start and end points
                &BLACK,
            ))?;
        }
    }


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

fn create_lines(points: &HashMap<char, Vec<Point>>) -> HashMap<char, Vec<Line>> {
    let lines: HashMap<char, Vec<Line>> = points.iter()
    .map( |(key,vec)| {
        let mut lines:Vec<Line> = vec![];
        for i in 0..vec.len() {
            for j in i+1..vec.len() {
                lines.push(Line::new(vec[i], vec[j]));
            }
        }
        (*key, lines)
    })
    .collect();
    return lines;
}

fn extend_line_to_bbox(line: &Line, bbox: (f64, f64, f64, f64) ) -> Line {
    let (x_min, y_min, x_max, y_max) = bbox;
    let slope = line.slope();
    let y0 = line.start.y;
    let x0 = line.start.x;
    // Calculate intercept
    let c = y0 - slope * x0;

    // Intersections
    let mut points = Vec::new();

    // Left edge (x = x_min)
    let y_left = slope * x_min + c;
    if y_min <= y_left && y_left <= y_max {
        points.push((x_min, y_left));
    }

    // Right edge (x = x_max)
    let y_right = slope * x_max + c;
    if y_min <= y_right && y_right <= y_max {
        points.push((x_max, y_right));
    }

    // Bottom edge (y = y_min)
    if slope != 0.0 {
        let x_bottom = (y_min - c) / slope;
        if x_min <= x_bottom && x_bottom <= x_max {
            points.push((x_bottom, y_min));
        }
    }

    // Top edge (y = y_max)
    if slope != 0.0 {
        let x_top = (y_max - c) / slope;
        if x_min <= x_top && x_top <= x_max {
            points.push((x_top, y_max));
        }
    }

    // Return the two intersection points
    if points.len() >= 2 {
        Line::new(points[0], points[1])
    } else {
        print!("Here!\n");
        println!("points:{:?}",points);
        //panic!("Line does not intersect two edges of the bounding box");
        *line

    }
}

fn extend_lines(lines: &HashMap<char, Vec<Line>>, x: i32, y: i32) -> HashMap<char, Vec<Line>> {
    let extended_map: HashMap<char, Vec<Line>> = lines.iter()
        .map(|(key, vec)| {
            let extended_vec: Vec<Line> = vec.iter() 
                .map(|line| {
                    extend_line_to_bbox(&line, (0.0,0.0,x as f64 + 1.,y as f64+ 1.))
                }).collect();
            (*key, extended_vec)
        }).collect();

    extended_map
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let path = String::from("custom_test.txt");

    let matrix = parse_input(&read_file(&path).unwrap()).unwrap();

    //Part 1
    let sum = 0;
    // create map Symbol -> Points[] by parsing the matrix
    let points = parse_map(&matrix);

    //Create Pair-Wise Lines between any two points belonging to a frequency
    let lines = create_lines(&points);

    //Extend lines to span the entire map
    let extended_lines = extend_lines(&lines, matrix[0].len() as i32, matrix.len() as i32);    

    draw_stuff(matrix[0].len(), matrix.len(), &points, &extended_lines)?;
    //Draw two circles for each line, one for each node defining the line
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
