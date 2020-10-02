use std::env;
use std::fs;
use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub struct InvalidInstruction;

impl fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid instruction")
    }
}

impl Error for InvalidInstruction {}


fn instruction_interpreter(instruction: &str, memory: &mut Vec<u8>, pointer: &mut usize) {
    if instruction == "moo" {
        moo_temp();
    } else if instruction == "mOo" {
        move_pointer_back_one(pointer);
    } else if instruction == "moO" {
        move_pointer_forward_one(pointer);
    } else if instruction == "mOO" {
        moo_temp();
    } else if instruction == "Moo" {
        moo_temp();
    } else if instruction == "MOo" {
        moo_temp();
    } else if instruction == "MoO" {
        increment_current_memory_address(memory, pointer);
    } else if instruction == "oom" {
        moo_temp();
    } else if instruction == "MOO" {
        moo_temp();
    } else if instruction == "OOO" {
        moo_temp();
    } else if instruction == "MMM" {
        moo_temp();
    } else if instruction == "OOM" {
        moo_temp();
    } else {
        panic!("Invalid instruction --> {}", instruction);
    }    
}

fn moo_temp() -> u32 {
    1u32
}

fn move_pointer_back_one(pointer: &mut usize){
    match pointer {
        0 => &mut 0,
        _ => &mut (*pointer - 1usize),
    };
}

fn move_pointer_forward_one(pointer: &mut usize) {
    &mut (*pointer + 1);
}

fn increment_current_memory_address(memory: &mut Vec<u8>, pointer: &mut usize) {
    if let Some(mem) = memory.get_mut(*pointer) {
        if mem == &255u8 {
            *mem = 0;
        } else {
            *mem += 1;
        }
    } else {
        panic!("Incrementing Memory Error")
    }
    
}

fn main() {
    let mut memory: Vec<u8> = vec!(0);
    let mut pointer: usize = 0;
    let valid_instructions = ["moo", "mOo", "moO", "mOO", "Moo", "MOo", "MoO", "MOO", "OOO", "MMM",
     "OOM", "oom"];
    let filename = match parse_arg() {
        Some(f) => f,
        None => {
            println!("Error: No filename entered, Exiting...");
            std::process::exit(1)
        }
    };
    let file: String = match read_file(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("File \'{}\' not found.\n{}", &filename, e);
            std::process::exit(1);
        }
    };
    let mut instructions: Vec<&str> = file.split_whitespace().collect();
    instructions.reverse();
    while let Some(instruction) = instructions.pop() {
        if valid_instructions.contains(&instruction){
            instruction_interpreter(instruction, &mut memory, &mut pointer);
            println!("{:?}", memory);
        } else {
            panic!("Invalid instruction: {}.", instruction);
        }
    }
}

fn parse_arg() -> Option<String> {
    Some(env::args().nth(1).expect("Error: No filename entered."))

}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}