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

#[derive(Debug)]
enum IncrementMode {
    Forward,
    Backward,
}


pub struct CowIncrementer {
    mode: IncrementMode,
}

impl CowIncrementer {
    fn next(&mut self, i: &mut usize) {
        match self.mode {
            IncrementMode::Forward => *i += 1,
            IncrementMode::Backward => *i -= 1,
        }
    }
}


pub struct InstructionInterpreter {
    memory: Vec<u8>,
    pointer: usize,
    incrementer: CowIncrementer,
    instructions: Vec<String>,
    i: usize,
    execute_instructions: bool,
}

impl InstructionInterpreter {
    fn cow(&mut self) {
        while self.i < self.instructions.len() {
            self.instruction_interpreter();
            self.incrementer.next(&mut self.i);
        }
    }

    fn instruction_interpreter(&mut self) {
        let instruction = &self.instructions[self.i];
        println!("{}", instruction);
        if self.execute_instructions == false {
            if instruction == "MOO" {
                self.execute_instructions = true;
                self.incrementer.mode = IncrementMode::Forward;
            } else {
               return
            }
        } else {
            if instruction == "moo" {
                // When finding a 'moo' you then search in reverse for a 'MOO', skipping
                // the instruction that is immediately before it
                self.i -= 1;
                self.incrementer.mode = IncrementMode::Backward;
                self.execute_instructions = false;
            } else if instruction == "mOo" {
                self.move_pointer_back_one();
            } else if instruction == "moO" {
                self.move_pointer_forward_one();
            } else if instruction == "mOO" {
                moo_temp();
            } else if instruction == "Moo" {
                self.input_or_read_ascii_value();
            } else if instruction == "MOo" {
                self.decrement_current_memory_address();
            } else if instruction == "MoO" {
                self.increment_current_memory_address();
            } else if instruction == "oom" {
                moo_temp();
            } else if instruction == "MOO" {
                moo_temp();
            } else if instruction == "OOO" {
                self.set_current_memory_to_zero();
            } else if instruction == "MMM" {
                moo_temp();
            } else if instruction == "OOM" {
                println!("{}", self.get_current_memory_as_integer());
            } else if instruction == "oom" {
                let integer: u8 = read_integer_from_stdin();
                self.set_memory_value_from_integer(integer);
            } else {
                panic!("Invalid instruction --> {}", instruction);
            }
        }
    }

    fn set_current_memory_to_zero(&mut self) {
        if let Some(mem) = self.memory.get_mut(self.pointer) {
            *mem = 0;
        } else {
            panic!("Could not set current memory address to zero.");
        }
    }
    
    
    fn get_current_memory_as_integer(&mut self) -> String {
        if let Some(mem) = self.memory.get(self.pointer) {
            format!("{}", mem)
        } else {
            panic!("An error occured whilst printing the current memory address.")
        }
    }

    fn move_pointer_forward_one(&mut self) {
        if self.pointer == self.memory.len()-1 {
            self.memory.push(0)
        }
        self.pointer += 1;
    }

    fn move_pointer_back_one(&mut self){
        match self.pointer {
            0 => self.pointer = 0,
            _ => self.pointer -= 1,
        };
    }
    
    
    fn increment_current_memory_address(&mut self) {
        if let Some(mem) = self.memory.get_mut(self.pointer) {
            if mem == &255u8 {
                *mem = 0;
            } else {
                *mem += 1;
            }
        } else {
            panic!("Incrementing Memory Error")
        }
    }
    
    
    fn decrement_current_memory_address(&mut self) {
        if let Some(mem) = self.memory.get_mut(self.pointer) {
            if mem == &0u8 {
                *mem = 255
            } else {
                *mem -= 1
            }
        }
    }


    fn set_memory_value_from_integer(&mut self, integer:u8) {
        if let Some(mem) = self.memory.get_mut(self.pointer) {
            *mem = integer
        } else {
            panic!("Couldn't write input integer to memory.")
        }
    }

    fn input_or_read_ascii_value(&mut self) {
        if let Some(mem) = self.memory.get_mut(self.pointer) {
            if mem == &0u8 {
                let input = get_single_character_from_stdin();
                let ascii_value = string_to_ascii_value(input);
                *mem = ascii_value
            } else {
                let ascii_value = get_utf8_from_integer(mem);
                println!("{}", ascii_value); 
            }
        }
    }
}


fn moo_temp() -> u32 {
    1u32
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


fn get_utf8_from_integer(n: &mut u8) -> String {
    // If the byte value is > 127, it needs to be subtracted by 128 to be in
    // valid ascii range
    if *n > 127u8 {
        *n -= 128;
    }
    String::from_utf8(vec!(*n)).unwrap()
}


fn parse_arg() -> Option<String> {
    Some(env::args().nth(1).expect("Error: No filename entered."))

}


fn read_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}


pub fn main() {
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
    let instructions: Vec<String> = file.split_whitespace().map(|s| s.to_owned()).collect();
    let mut cow = InstructionInterpreter{
        memory: vec!(0),
        pointer: 0usize,
        incrementer: CowIncrementer{mode: IncrementMode::Forward},
        instructions: instructions,
        i: 0,
        execute_instructions: true,
    };
    cow.cow();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_pointer_forward_one() {
        let mut cow = InstructionInterpreter{
            memory: vec!(0),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.move_pointer_forward_one();
        assert_eq!(1usize, cow.pointer);
        assert_eq!(vec!(0, 0), cow.memory);
    }

    #[test]
    fn test_move_pointer_back_one() {
        let mut cow = InstructionInterpreter{
            memory: vec!(0),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.move_pointer_back_one();
        assert_eq!(0usize, cow.pointer);
    }

    #[test]
    fn test_increment_current_memory_address() {
        let mut cow = InstructionInterpreter{
            memory: vec!(0),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.increment_current_memory_address();
        assert_eq!(cow.memory, vec!(1));
    }

    #[test]
    fn test_decrement_current_memory_address() {
        let mut cow = InstructionInterpreter{
            memory: vec!(1),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.decrement_current_memory_address();
        assert_eq!(cow.memory, vec!(0));
    }

    #[test]
    fn test_set_current_memory_address_to_zero() {
        let mut cow = InstructionInterpreter{
            memory: vec!(1),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.set_current_memory_to_zero();
        assert_eq!(cow.memory, vec!(0));
    }

    #[test]
    fn test_print_current_memory_address_as_integer() {
        let mut cow = InstructionInterpreter{
            memory: vec!(3),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        assert_eq!(
            cow.get_current_memory_as_integer(),
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
        let mut cow = InstructionInterpreter{
            memory: vec!(0),
            pointer: 0usize,
            incrementer: CowIncrementer{mode: IncrementMode::Forward},
            instructions: vec!("test".to_owned()),
            i: 0,
            execute_instructions: true,
        };
        cow.set_memory_value_from_integer(4);
        assert_eq!(cow.memory, vec!(4));
    }

    #[test]
    fn test_read_ascii_value() {
        let test = get_utf8_from_integer(&mut 103);
        assert_eq!("g".to_owned(), test)
    }
}