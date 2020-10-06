use std::env;
use std::fs;
use std::fmt;
use std::error::Error;
use std::io;


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
        move_pointer_forward_one(memory, pointer);
    } else if instruction == "mOO" {
        moo_temp();
    } else if instruction == "Moo" {
        input_or_read_ascii_value(memory, pointer);
    } else if instruction == "MOo" {
        decrement_current_memory_address(memory, pointer);
    } else if instruction == "MoO" {
        increment_current_memory_address(memory, pointer);
    } else if instruction == "oom" {
        moo_temp();
    } else if instruction == "MOO" {
        moo_temp();
    } else if instruction == "OOO" {
        set_current_memory_to_zero(memory, pointer);
    } else if instruction == "MMM" {
        moo_temp();
    } else if instruction == "OOM" {
        println!("{}", get_current_memory_as_integer(memory.to_vec(), *pointer));
    } else if instruction == "oom" {
        let integer: u8 = read_integer_from_stdin();
        set_memory_value_from_integer(integer, memory, pointer);
    } else {
        panic!("Invalid instruction --> {}", instruction);
    }    
}


fn read_integer_from_stdin() -> u8 {
    loop {
        let mut buffer = String::new();
        let input = match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => panic!("Error reading from stdin: {}", e),
        };
        let integer = match input.parse::<u8>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid integer, try again,");
                continue
            }
        };
        return integer;
    }
}


fn set_memory_value_from_integer(integer:u8, memory: &mut Vec<u8>, pointer: &mut usize) {
    if let Some(mem) = memory.get_mut(*pointer) {
        *mem = integer
    } else {
        panic!("Couldn't write input integer to memory.")
    }
}


fn get_single_character_from_stdin() -> String {
    loop {
        let mut buffer = String::new();
        let input = match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => panic!("Error reading from stdin: {}", e),
        };
        if input.len() > 2 {
            println!("You can only enter a single character.");
            continue
        } else if input.len() <= 1 {
            println!("You need to enter a character.");
            continue
        } else {
            return input;
        }
    }
}


fn string_to_ascii_value(input: String) -> u8 {
    if input.is_ascii() {
        let bytes = input.as_bytes();
        return *bytes.first().unwrap()
    } else {
        panic!("bytes conversion broke")
    }
}

fn get_utf8_from_integer(n: u8) -> String {
    String::from_utf8(vec!(n)).unwrap()
}


fn input_or_read_ascii_value(memory: &mut Vec<u8>, pointer: &mut usize) {
    if let Some(mem) = memory.get_mut(*pointer) {
        if mem == &0u8 {
            let input = get_single_character_from_stdin();
            let ascii_value = string_to_ascii_value(input);
            *mem = ascii_value
        } else {
            let ascii_value = get_utf8_from_integer(*mem);
            println!("{}", *mem);
            println!("{}", ascii_value); 
        }
    }
}


fn set_current_memory_to_zero(memory: &mut Vec<u8>, pointer: &mut usize) {
    if let Some(mem) = memory.get_mut(*pointer) {
        *mem = 0;
    } else {
        panic!("Could not set current memory address to zero.");
    }
}


fn get_current_memory_as_integer(memory: Vec<u8>, pointer: usize) -> String{
    if let Some(mem) = memory.get(pointer) {
        format!("{}", mem)
    } else {
        panic!("An error occured whilst printing the current memory address.")
    }
}


fn moo_temp() -> u32 {
    1u32
}


fn move_pointer_back_one(pointer: &mut usize){
    match pointer {
        0 => *pointer = 0,
        _ => *pointer -= 1,
    };
}


fn move_pointer_forward_one(memory: &mut Vec<u8>, pointer: &mut usize) {
    if *pointer == memory.len()-1 {
        memory.push(0)
    }
    *pointer += 1;
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


fn decrement_current_memory_address(memory: &mut Vec<u8>, pointer: &mut usize) {
    if let Some(mem) = memory.get_mut(*pointer) {
        if mem == &0u8 {
            *mem = 255
        } else {
            *mem -= 1
        }
    }
}


pub fn main() {
    let mut memory: Vec<u8> = vec!(0);
    let mut pointer: usize = 0;
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
        instruction_interpreter(instruction, &mut memory, &mut pointer);
    }
}

fn parse_arg() -> Option<String> {
    Some(env::args().nth(1).expect("Error: No filename entered."))

}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_pointer_forward_one() {
        let mut pointer: usize = 0;
        let mut memory: Vec<u8> = vec!(0);
        move_pointer_forward_one(&mut memory, &mut pointer);
        assert_eq!(1usize, pointer);
        assert_eq!(vec!(0, 0), memory);
    }

    #[test]
    fn test_move_pointer_back_one() {
        let mut pointer: usize = 1;
        move_pointer_back_one(&mut pointer);
        assert_eq!(0usize, pointer);
    }

    #[test]
    fn test_increment_current_memory_address() {
        let mut pointer: usize = 0;
        let mut memory: Vec<u8> = vec!(0);
        increment_current_memory_address(&mut memory, &mut pointer);
        assert_eq!(memory, vec!(1));
    }

    #[test]
    fn test_decrement_current_memory_address() {
        let mut pointer: usize = 0;
        let mut memory: Vec<u8> = vec!(1);
        decrement_current_memory_address(&mut memory, &mut pointer);
        assert_eq!(memory, vec!(0));
    }

    #[test]
    fn test_set_current_memory_address_to_zero() {
        let mut pointer: usize = 0;
        let mut memory: Vec<u8> = vec!(1);
        set_current_memory_to_zero(&mut memory, &mut pointer);
        assert_eq!(memory, vec!(0));
    }

    #[test]
    fn test_print_current_memory_address_as_integer() {
        let pointer: usize = 0;
        let memory: Vec<u8> = vec!(3);
        assert_eq!(
            get_current_memory_as_integer(memory, pointer),
            "3".to_owned()
        );
    }

    #[test]
    fn test_string_to_ascii_value() {
        let a_string = String::from("g");
        let ascii_a_value = 103;
        let string_to_ascii = string_to_ascii_value(a_string);
        assert_eq!(string_to_ascii, ascii_a_value);
    }

    #[test]
    fn test_set_memory_value_from_integer() {
        let mut pointer: usize = 0;
        let mut memory: Vec<u8> = vec!(0);
        set_memory_value_from_integer(4, &mut memory, &mut pointer);
        assert_eq!(memory, vec!(4));
    }

    #[test]
    fn test_read_ascii_value() {
        let test = get_utf8_from_integer(103);
        assert_eq!("g".to_owned(), test)
    }
}