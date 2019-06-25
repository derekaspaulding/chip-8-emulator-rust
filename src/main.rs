mod interpreter;
mod instruction;

use instruction::Instruction;
use std::io;
use std::u8;

// This main function is a placeholder to give the project some functionality until the interpreter is done.
// It reads in a 2 byte value in hexadecimal and prints the resulting instruction.
fn main() {
    let keep_going = true;

    while keep_going {
        let mut input = String::new();

        println!("Enter 2 byte hexadecimal instruction with space between bytes. Example: F5 07");
        io::stdin().read_line(&mut input).ok().expect("Could not read input");

        let raw_instruction = to_raw_instruction(&input.trim()).expect("Could not parse instruction");

        println!("{:?}\n\n", Instruction::parse(raw_instruction));
    }
}

fn to_raw_instruction(input_string: &str) -> Option<(u8, u8)>  {
    if input_string.len() != 5 {
        return None;
    }

    let bytes_as_str: Vec<&str> = input_string.split(" ").collect();

    if bytes_as_str.len() != 2 {
        return None;
    }

    // TODO: This could have better error handling, but this implementation probably won't be used
    // when reading from a binary file so it is fine for now.
    let number_values: Vec<u8> = bytes_as_str
        .into_iter()
        .map(|byte| u8::from_str_radix(byte, 16).unwrap())
        .collect();

    Some((
            number_values[0],
            number_values[1],
        ))
}
