use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::VecDeque;

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
        Ok(_) => (),
    }
    let (layout, instructions) = s.split_at(s.find("\n\n").unwrap_or_else(| | {
        panic!("Couldn't find delimiter between the layout and the instructions. Input format might be wrong!")}
    ));
    let mut yard = to_vector(layout);
    println!("----------------Initial State----------------");
    print_state(&yard);
    println!("----------------Final   State----------------");
    execute_instructions(instructions, &mut yard);
    
    print_state(&yard);

    let result = get_top_crate_arrangement_string(&yard);
    println!("The result is {result}");
}

fn to_vector(layout: &str) -> VecDeque<VecDeque<char>> {
    let yard_size = yard_size(layout);
    let mut yard: VecDeque<VecDeque<char>> = create_yard(yard_size);
    for line in layout.lines(){
        for (j, &item) in line.as_bytes().iter().enumerate() {
            if j % 4 == 1 && item != b' ' {
                yard[j/4].push_front(char::from(item));
            }
        }
    }
    return yard;
}

fn yard_size(layout: &str) -> usize {
    let size = layout.find('\n');
    let size = size.unwrap_or_else(|| {
        panic!("Could not determine yard size");
    }) / 4;
    return size + 1;
}

fn create_yard(mut size: usize) -> VecDeque<VecDeque<char>> {
    let mut yard: VecDeque<VecDeque<char>> = VecDeque::new();
    loop {
        yard.push_front(VecDeque::new());
        size -= 1;
        if size == 0 {
            break;
        }
    }
    return yard;
}

fn execute_instructions(instructions: &str, yard_state: &mut VecDeque<VecDeque<char>>) {
    for line in instructions.trim().lines() {
        execute_instruction(decode_instruction(line), yard_state);
    }
}

fn get_next_num(string: &str) -> (u32, &str) {
    let first_occ = string.find(char::is_numeric).unwrap_or_else( | | {
        panic!("Found no numeric-type char in string \"{}\"\n",string);
    });

    let mut last_occ = first_occ;
    while string.chars().nth(last_occ+1).unwrap_or_else(|| 'X' ).is_numeric() {
        last_occ += 1;
    };

    let num: u32 = string.get(first_occ..last_occ+1).unwrap_or_else(|| {
        panic!("Could not parse string to numbers!\n");
    }).parse().unwrap_or_else(|err| {
        panic!("{err}");
    });
    let newstr = string.get(last_occ+1..).unwrap_or_else(||{
        panic!("Could not clean up string!");
    });
    ( num , newstr)
}

fn decode_instruction(instruction: &str) -> (u32, u32, u32) {
    let mut instr = instruction;
    let num;
    let from;
    let to;
    (num , instr) = get_next_num(&instr);
    (from , instr) = get_next_num(&instr);
    (to , _) = get_next_num(&instr);
    return (num, from, to);
}

fn execute_instruction((num, from, to) : (u32,u32,u32),yard: &mut VecDeque<VecDeque<char>>) {
    let mut i = num;
    let from: usize = from as usize;
    let to: usize = to as usize;

    while i > 0 {
        i -= 1;
        let container = yard[from-1].pop_back().unwrap_or_else(|| {
            panic!("Could not pop from stack {from}!\n");
        });
        yard[to-1].push_back(container);
    }
}

fn get_top_crate_arrangement_string(yard: &VecDeque<VecDeque<char>>) -> String {
    let mut string = String::new();
    for stack in yard {
        string.push(*stack.back().unwrap_or_else(|| &' '));
    }
    return string;
}

fn print_state(yard: &VecDeque<VecDeque<char>>) {
    let biggest_stack_size = get_biggest_stack_size(yard);
    let mut output_string = String::new();
    for i in (1..biggest_stack_size).rev() {
        for stack in yard.iter() {
            if i < stack.len() {
                output_string.push_str(  &format!("[{}] ", stack[i]));
            } else {
                output_string.push_str("    ");
            }
            
        }
        output_string.push('\n');
    }
    for i in 1..yard.len()+1 {
        output_string.push_str(  &format!(" {}  ", i));
    }
    print!("{}\n",output_string);
}

fn get_biggest_stack_size(yard: &VecDeque<VecDeque<char>>) -> usize {
    let mut biggest_stack_size = 0;
    for stack in yard {
        let stack_size = stack.len();
        if biggest_stack_size < stack_size {
            biggest_stack_size = stack_size;
        }
    }
    return biggest_stack_size;
}



