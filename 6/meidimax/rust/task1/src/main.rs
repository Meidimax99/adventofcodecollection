use std::fs::File;
use std::io::{prelude::*};
use std::path::Path;

const MARKER_LENGTH: usize = 4;

fn main() {
    let path = Path::new("../input.txt");
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
    let index = get_marker_index(s);
    print!("The index of the messages start is {}.\n", index.unwrap_or(0));
}

fn get_marker_index(s: String) -> Option<usize> {
    let mut chars: [char; MARKER_LENGTH] = ['\0';MARKER_LENGTH];
    for (i , c) in s.chars().into_iter().enumerate() {
        chars[i % MARKER_LENGTH] = c;
        if i >= 3 && all_different(chars) {
            return Some(i+1);
        }
    }
    None
}

fn all_different(chars: [char; MARKER_LENGTH]) -> bool {
    for i in 0..MARKER_LENGTH {
        for j in i+1..MARKER_LENGTH {
            if chars[i] == chars[j]  {
                return false;
            }
        }
    }
    return true;
}


