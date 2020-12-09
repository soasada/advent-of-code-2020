use files::load_str_vec_from;
use std::collections::HashMap;

pub fn problem1() -> i64 {
    let instructions_loaded = load_str_vec_from("crates/day8/src/day8_input.txt");

    if let Ok(instructions) = instructions_loaded {
        return execute_program(&instructions).0;
    }

    0
}

pub fn problem2() -> i64 {
    let instructions_loaded = load_str_vec_from("crates/day8/src/day8_input.txt");

    if let Ok(instructions) = instructions_loaded {
        let mut result = execute_program(&instructions);
        for (i, instruction) in instructions.iter().enumerate() {
            if result.1 >= instructions.len() {
                break;
            }

            let decoded_instruction: Vec<&str> = instruction.split(" ").collect();
            let mut operation = decoded_instruction[0];

            if operation == "jmp" {
                operation = "nop";
            } else if operation == "nop" {
                operation = "jmp";
            }

            if operation != "acc" {
                let mut new_program = instructions.to_owned();
                new_program[i] = operation.to_owned() + " " + decoded_instruction[1];
                result = execute_program(&new_program);
            }
        }

        return result.0;
    }

    0
}

fn execute_program(program: &Vec<String>) -> (i64, usize) {
    let mut accumulator = 0;
    let mut offset = 0;
    let mut executed_instructions = HashMap::new();

    while !executed_instructions.contains_key(&offset) && offset < program.len() {
        let instruction = program.get(offset).unwrap();
        let decoded_instruction: Vec<&str> = instruction.split(" ").collect();
        let operation = decoded_instruction[0];
        let argument = decoded_instruction[1].parse::<i64>().unwrap();

        executed_instructions.insert(offset, (operation, argument));

        if operation == "acc" {
            accumulator += argument;
            offset += 1;
        } else if operation == "jmp" {
            if argument.is_negative() {
                offset = offset - argument.wrapping_abs() as u32 as usize;
            } else {
                offset += argument as usize;
            }
        } else {
            offset += 1;
        }
    }

    return (accumulator, offset);
}