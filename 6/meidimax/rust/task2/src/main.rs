use std::fs::File;
use std::io::{prelude::*};
use std::path::Path;

const PACKET_MARKER_LENGTH: usize = 4;
const MESSAGE_MARKER_LENGTH: usize = 14;


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
    let packet_index = get_marker_index(&s,PACKET_MARKER_LENGTH);
    let message_index = get_marker_index(&s,MESSAGE_MARKER_LENGTH);
    print!("The index of the packet start is {}.\n", packet_index.unwrap_or(0));
    print!("The index of the packet start is {}.\n", message_index.unwrap_or(0));
}

fn get_marker_index(s: &String, marker_size: usize) -> Option<usize> {
    let mut chars: Vec<char> = vec!['\0'; marker_size];
    for (i , c) in s.chars().into_iter().enumerate() {
        chars[i % marker_size] = c;
        if i >= marker_size-1 && all_different(&chars, marker_size) {
            return Some(i+1);
        }
    }
    None
}

fn all_different(chars: &Vec<char>, marker_size: usize) -> bool {
    for i in 0..marker_size {
        for j in i+1..marker_size {
            if chars[i] == chars[j]  {
                return false;
            }
        }
    }
    return true;
}


