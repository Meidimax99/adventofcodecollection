use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{usize, vec};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::sync::Arc;

use std::thread;


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
    loop {
        original_program.push(elem as usize);
        let mut instr = Instr{opcode: OpCode::Adv, operant: 0};
        instr.opcode = OpCode::from_u8(elem).unwrap();
        instr.operant = nums_iter.next().unwrap();
        original_program.push(instr.operant as usize);
        let next = nums_iter.next();
        instructions.push(instr);
        if next.is_some() {
            elem = next.unwrap();
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

#[derive(Debug, FromPrimitive)]
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

#[derive(Debug,Clone)]
struct Regs {
    a: usize,
    b: usize,
    c: usize
}
#[derive(Debug)]

struct Instr {
    opcode: OpCode,
    operant: u8,
}


impl Instr {
    // A method associated with the enum
    fn execute(&self, regs: &mut Regs, pc: &mut usize) -> Option<usize>{
        match self.opcode {
            OpCode::Adv => {
                let numerator = regs.a;
                let denominator = (2 as usize).pow(self.get_op_combo(regs) as u32);
                regs.a = (numerator as f64 / denominator as f64) as usize;
                *pc += 1;
                None
            },
            OpCode::Bxl => {
                regs.b = regs.b ^ self.operant as usize;
                *pc += 1;
                None
            },
            OpCode::Bst => {
                let comb = self.get_op_combo(regs) % 8;
                regs.b = comb;
                *pc += 1;
                None
            },
            OpCode::Jnz => {
                if regs.a == 0 {
                    *pc += 1;
                } else {
                    *pc = self.operant as usize;
                }
                None
            },
            OpCode::Bxc => {
                regs.b = regs.b ^ regs.c;
                *pc += 1;
                None
            },
            OpCode::Out => {
                let out = self.get_op_combo(regs) % 8;
                *pc += 1;
                Some(out)
            },
            OpCode::Bdv => {
                let numerator = regs.a;
                let denominator = (2 as usize).pow(self.get_op_combo(regs) as u32);
                regs.b = (numerator as f64 / denominator as f64) as usize;
                *pc += 1;
                None
            },
            OpCode::Cdv => {
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

fn run(regs: &mut Regs, instructions: &Vec<Instr>) -> Vec<usize> {
    let mut pc = 0;
    let mut res: Vec<usize> = vec![];


    loop {
        let  next_instr: &Instr = &instructions[pc];
    
        let opt = next_instr.execute(regs, &mut pc);
        let output = opt.is_some();
        if output {
            //print!("{}",opt.unwrap());
            res.push(opt.unwrap());
        }
        if pc >= instructions.len() {
            break;
        }
        // if output {
        //     print!(",");
        // }
    }
    return res;
}

fn vec_equ(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    //println!("A: {:?}\nB: {:?}", a, b);
    //let a = vec![2, 4, 1, 3, 7, 5, 0, 3, 4, 3, 1, 5, 5, 5, 3, 0];
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
} 

fn run_thread(regs: Arc<Regs>, instructions: Arc<Vec<Instr>>, start: usize, n_threads: usize, original_prog: Arc<Vec<usize>>) {
    let mut a = start;

    loop {
        let mut regs_copy = (*regs).clone();
        regs_copy.a = a;
        //println!("Trying {}",a);
        let res = run(&mut regs_copy, &*instructions);
        if vec_equ(&res, &*original_prog) {
            println!("{} - {:?}", a, res);
            break;
        }
        a += n_threads;
    }
}
fn main() {

    let path = String::from("input.txt");

    let ( regs, instructions, original_prog) = parse_input(&read_file(&path).unwrap()).unwrap();

    let mut handles = vec![];
    let n_threads = 8;

    let shared_regs = Arc::new(regs);
    let shared_instr = Arc::new(instructions);
    let shared_org = Arc::new(original_prog);

    for i in 0..=n_threads {

        let shared_regs_clone = Arc::clone(&shared_regs);
        let shared_instr_clone = Arc::clone(&shared_instr);
        let shared_org_clone = Arc::clone(&shared_org);

        let handle = thread::spawn(move || {
            run_thread(shared_regs_clone, shared_instr_clone, i, n_threads, shared_org_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    println!()

}
