use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn contains(range1: &Vec<u32>, range2: &Vec<u32>) -> bool {
    return range2[0] >= range1[0] && range2[1] <= range1[1];
}

fn any_overlaps(range1: &Vec<u32>, range2: &Vec<u32>) -> bool {
    let rng1 = range1[0]..range1[1]+1;
    let rng2 = range2[0]..range2[1]+1;
    return rng1.contains(&range2[0]) || rng1.contains(&range2[1]) || rng2.contains(&range1[0]) || rng2.contains(&range1[1]);
}

fn one_range_fully_contains_other(range1: &Vec<u32>, range2: &Vec<u32>) -> bool {
    return contains(&range1, &range2) || contains(&range2, &range1);
}
fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(bytes) => print!("Successfully read {bytes} Bytes from {display}\n"),
    };

    let lines = s.lines();

    let mut sum_included_ranges = 0;
    let mut any_overlaps_count = 0;
    for line in lines {
        let v: Vec<u32> = line.split(&['-', ',']).map(|s| s.parse::<u32>().unwrap()).collect();
        let range1 = Vec::from([v[0],v[1]]);
        let range2 = Vec::from([v[2],v[3]]);
        if one_range_fully_contains_other(&range1, &range2) {
            sum_included_ranges += 1;
        } 
        if any_overlaps(&range1, &range2) {
            any_overlaps_count += 1;
        }
    }
    println!("Found {sum_included_ranges} complete overlaps!");
    println!("Found {any_overlaps_count}  partial overlaps!");
}
