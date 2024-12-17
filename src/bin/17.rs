use std::str::FromStr;
advent_of_code::solution!(17);

struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

fn parse_input(input: &str) -> (Registers, Vec<usize>) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Register A:") {
            a = parse_value(line, "Register A:");
        } else if line.starts_with("Register B:") {
            b = parse_value(line, "Register B:");
        } else if line.starts_with("Register C:") {
            c = parse_value(line, "Register C:");
        } else if line.starts_with("Program:") {
            let program_str = line.strip_prefix("Program:").unwrap().trim();
            program = program_str
                .split(',')
                .map(|num| usize::from_str(num.trim()).unwrap())
                .collect();
        }
    }

    (
        Registers { a, b, c },
        program,
    )
}

/// Helper function to parse a numeric value from a line with a prefix.
fn parse_value(line: &str, prefix: &str) -> i64 {
    line.strip_prefix(prefix)
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap()
}

fn get_combo_value(operand: usize, registers: &Registers) -> i64 {
    match operand {
        0..=3 => operand as i64, // Literal values 0-3
        4 => registers.a,        // Register A
        5 => registers.b,        // Register B
        6 => registers.c,        // Register C
        _ => panic!("Invalid combo operand: 7"), // Operand 7 is invalid
    }
}

fn run_program(registers: &mut Registers, program: &[usize]) -> Vec<String> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program.get(instruction_pointer + 1).copied().unwrap_or(0);

        match opcode {
            0 => { // adv
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.a /= denominator;
            }
            1 => { // bxl
                registers.b ^= operand as i64;
            }
            2 => { // bst
                registers.b = get_combo_value(operand, registers) % 8;
            }
            3 => { // jnz
                if registers.a != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            }
            4 => { // bxc
                registers.b ^= registers.c;
            }
            5 => { // out
                output.push((get_combo_value(operand, registers) % 8).to_string());
            }
            6 => { // bdv
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.b = registers.a / denominator;
            }
            7 => { // cdv
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.c = registers.a / denominator;
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }

        // Advance the instruction pointer unless modified by jnz
        instruction_pointer += 2;
    }

    output
}

fn run_program_2(registers: &mut Registers, program: &[usize]) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program.get(instruction_pointer + 1).copied().unwrap_or(0);

        match opcode {
            0 => {
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.a /= denominator;
            }
            1 => {
                registers.b ^= operand as i64;
            }
            2 => {
                registers.b = get_combo_value(operand, registers) % 8;
            }
            3 => {
                if registers.a != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            }
            4 => {
                registers.b ^= registers.c;
            }
            5 => {
                output.push((get_combo_value(operand, registers) % 8) as usize);
            }
            6 => {
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.b = registers.a / denominator;
            }
            7 => {
                let denominator = 2_i64.pow(get_combo_value(operand, registers) as u32);
                registers.c = registers.a / denominator;
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }

        instruction_pointer += 2;
    }

    output
}

fn find_min_a_for_program_output(program: &[usize]) -> i64 {
    let mut a = 1;
    loop {
        let mut registers = Registers { a, b: 0, c: 0 };
        let output = run_program_2(&mut registers, program);
            
        if output == program {
            return a;
        }

        a += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut registers, program) = parse_input(input);

    let output = run_program(&mut registers, &program);
    
    println!("{:?}", output);
    Some(0)
}


pub fn part_two(input: &str) -> Option<u32> {
    let (_, program) = parse_input(input);
    let lowest_a = find_min_a_for_program_output(&program);
    
    println!("{:?}", lowest_a);
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
