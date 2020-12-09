use files::load_str_vec_from;
use std::collections::HashMap;

pub fn problem1() -> i64 {
    let instructions_loaded = load_str_vec_from("crates/day8/src/day8_input.txt");

    if let Ok(instructions) = instructions_loaded {
        let mut accumulator = 0;
        let mut offset = 0;
        let mut executed_instructions = HashMap::new();

        while !executed_instructions.contains_key(&offset) {
            let instruction = instructions.get(offset).unwrap();
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

        return accumulator;
    }

    0
}

pub fn problem2() -> i64 {
    0
}