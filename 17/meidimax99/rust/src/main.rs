use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use std::usize;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


fn get_number(string: &str) -> usize {

    let chars = string.chars();

    let num: String = chars.into_iter().filter(|c| c.is_numeric()).collect();

    num.parse::<usize>().unwrap_or(0)
}

fn parse_input(contents: &String) -> Result<(Regs, Vec<Instr>, Vec<usize>), String> {
    
    let mut regs = Regs {a:0 ,b:0, c:0};
    let mut instructions: Vec<Instr> = vec![];

    let mut original_program: Vec<usize> = vec![];

    let mut lines = contents.lines();

    let mut line = lines.next().unwrap();

    regs.a = get_number(line);
    line = lines.next().unwrap();

    regs.b = get_number(line);
    line = lines.next().unwrap();
    
    regs.c = get_number(line);
    lines.next().unwrap();
    line = lines.next().unwrap();


    let nums: Vec<u8>= line.chars().into_iter().filter(|c| c.is_numeric()).map(|c| c as u8 - ('0' as u8)).collect();

    let mut nums_iter = nums.into_iter();
    let mut elem = nums_iter.next().unwrap();
    let mut index = 0;
    loop {
        original_program.push(elem as usize);
        let entry: Vec<i32> = vec![index-1];
        let mut instr = Instr{opcode: OpCode::Adv, operant: 0, index: index as u8, entry};
        instr.opcode = OpCode::from_u8(elem).unwrap();
        instr.operant = nums_iter.next().unwrap();
        original_program.push(instr.operant as usize);
        let next = nums_iter.next();
        instructions.push(instr);
        if next.is_some() {
            elem = next.unwrap();
            index += 1;
        } else {
            break;
        }

    }
    return Ok((regs, instructions, original_program));
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

#[derive(Debug, FromPrimitive, PartialEq)]
enum OpCode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv

}

#[derive(Debug)]
struct Regs {
    a: usize,
    b: usize,
    c: usize
}
#[derive(Debug)]

struct Instr {
    opcode: OpCode,
    operant: u8,
    index: u8,
    entry: Vec<i32>,
}


impl Instr {
    // A method associated with the enum
    fn execute(&self, regs: &mut Regs, pc: &mut usize) -> Option<usize>{
        match self.opcode {
            OpCode::Adv => {
                // Divide value in register by 2 to the power of the combo value, store to a
                let numerator = regs.a;
                let denominator = (2 as usize).pow(self.get_op_combo(regs) as u32);
                regs.a = (numerator as f64 / denominator as f64) as usize;
                *pc += 1;
                None
            },
            OpCode::Bxl => {
                // Xor B with operant and store it to b
                regs.b = regs.b ^ self.operant as usize;
                *pc += 1;
                None
            },
            OpCode::Bst => {
                // Store Combo value % 8 to b
                let comb = self.get_op_combo(regs) % 8;
                regs.b = comb;
                *pc += 1;
                None
            },
            OpCode::Jnz => {
                //Program counter to operant
                if regs.a == 0 {
                    *pc += 1;
                } else {
                    *pc = self.operant as usize;
                }
                None
            },
            OpCode::Bxc => {
                //Xor b and c and store to b
                regs.b = regs.b ^ regs.c;
                *pc += 1;
                None
            },
            OpCode::Out => {
                //Print
                let out = self.get_op_combo(regs) % 8;
                *pc += 1;
                Some(out)
            },
            OpCode::Bdv => {
                //Same as adv but store to b
                let numerator = regs.a;
                let denominator = (2 as usize).pow(self.get_op_combo(regs) as u32);
                regs.b = (numerator as f64 / denominator as f64) as usize;
                *pc += 1;
                None
            },
            OpCode::Cdv => {
                //Same as adv but store to c
                let numerator = regs.a;
                let denominator = (2 as usize).pow(self.get_op_combo(regs) as u32);
                regs.c = (numerator as f64 / denominator as f64) as usize;
                *pc += 1;
                None
            },
        }
    }

    fn get_op_combo(&self, regs: &Regs) -> usize{
        match self.operant {
            0..=3 => self.operant as usize,
            4 => regs.a,
            5 => regs.b,
            6 => regs.c,
            _ => 0

        }
    }
}

fn run(regs: &mut Regs, instructions: Vec<Instr>) {
    let mut pc = 0;

    loop {
        let  next_instr: &Instr = &instructions[pc];
    
        let opt = next_instr.execute(regs, &mut pc);
        //println!("-:{}:-",pc);
        let output = opt.is_some();
        if output {
            print!("{}",opt.unwrap());
        }
        if pc >= instructions.len() {
            break;
        }
        if output {
            print!(",");
        }
    }
}

fn find_instr(instructions: &Vec<Instr>, instr_type: OpCode) -> Vec<u8> {

    let mut instr_idx = vec![];

    for (idx, ele) in instructions.iter().enumerate() {
        if ele.opcode == instr_type {
            instr_idx.push(idx as u8);
        }
    }
    return instr_idx;
}

fn reverse_engineer(instructions: &mut Vec<Instr>, desired_output: &Vec<usize>) -> Option<usize> {
    // //Assumption: Only one Print Statement

    // //Find Print statements
    // let print_statements_idx = find_instr(&instructions, OpCode::Out);
    // assert!(print_statements_idx.len() == 1);
    // //Find jump statements
    // let jump_st_idx = find_instr(&instructions, OpCode::Jnz);

    // //Add entry information to jumps
    // for idx in jump_st_idx {
    //     let target = instructions[idx as usize].op;
    // }
    None
}
fn main() {

    let path = String::from("input.txt");

    let (mut regs, mut instructions, prog) = parse_input(&read_file(&path).unwrap()).unwrap();

    //run(&mut regs, instructions);

    println!();
    let a  = reverse_engineer(&mut instructions, &prog).unwrap_or(0);

    println!("A = {} lets the program replicate itself", a);

}
