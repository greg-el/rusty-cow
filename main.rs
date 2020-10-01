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

#[derive(Debug)]
pub struct OutsideStartOfMemory;

impl fmt::Display for OutsideStartOfMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pointer moved outside of start of memory.")
    }
}

impl Error for OutsideStartOfMemory {}


#[derive(Debug)]
pub struct OutsideEndOfMemory;

impl fmt::Display for OutsideEndOfMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pointer moved outside of end of memory.")
    }
}

impl Error for OutsideEndOfMemory {}


fn instruction_interpreter(instruction: &str) {
    let mut memory: Vec<u32> = Vec::new();
    let mut pointer: u32 = 0;
    if instruction == "moo" {
        moo_temp();
    } else if instruction == "mOo" {
        match move_pointer_back_one(&pointer) {
            Ok(p) => pointer = p,
            Err(e) => panic!("{}", e),
        }
    } else if instruction == "moO" {
        let memory_length = memory.len();
        match move_pointer_forward_one(&pointer, memory_length) {
            Ok(p) => pointer = p,
            Err(e) => panic!("{}", e),
        }
    } else if instruction == "mOO" {
        moo_temp();
    } else if instruction == "Moo" {
        moo_temp();
    } else if instruction == "MOo" {
        moo_temp();
    } else if instruction == "MoO" {
        increment_current_memory_address(&memory.as_slice(), &pointer)
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

fn move_pointer_back_one(pointer: &u32) -> Result<u32, OutsideStartOfMemory> {
    match pointer {
        0 => Err(OutsideStartOfMemory),
        _ => Ok(pointer - 1),
    }
}

fn move_pointer_forward_one(&pointer: &u32, length: usize) -> Result<u32, OutsideEndOfMemory> {
    match pointer {
        length => Err(OutsideEndOfMemory),
        _ => Ok(pointer + 1) 
    }
}

fn increment_current_memory_address(&memory: &[u32], &pointer: &u32) -> u32 {
    1u32
}

fn main() {
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
            instruction_interpreter(instruction);
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