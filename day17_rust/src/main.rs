use std::env;
use std::fs::File;
use std::io::prelude::*;

fn combo_operand(program: &[i64], address: usize, registers: &[i64]) -> usize {
    let raw_value: usize = program[address].try_into().unwrap();
    if raw_value <= 3 {
        raw_value
    } else if raw_value <= 6 {
        registers[raw_value - 4].try_into().unwrap()
    } else {
        panic!("Invalid combo operand {raw_value}");
    }
}

fn run(program: &[i64], registers: &mut [i64]) -> Vec<i64> {
    let mut pc: usize = 0;
    let mut outputs = Vec::<i64>::new();
    loop {
        if pc >= program.len() {
            break;
        }
        let opcode = program[pc];
        match opcode {
            0 => {
                /* adv */
                let denom = 1 << combo_operand(program, pc + 1, registers);
                registers[0] /= denom;
                pc += 2;
            }
            1 => {
                /* bxl */
                registers[1] ^= program[pc + 1];
                pc += 2;
            }
            2 => {
                /* bst */
                let operand: usize = combo_operand(program, pc + 1, registers) % 8;
                registers[1] = operand.try_into().unwrap();
                pc += 2;
            }
            3 => {
                /* jnz */
                if registers[0] == 0 {
                    pc += 2;
                } else {
                    pc = program[pc + 1].try_into().unwrap();
                }
            }
            4 => {
                /*bxc */
                registers[1] ^= registers[2];
                pc += 2;
                /* Unclear if this should exit if the operand is outside the program. Instructions say
                we should halt if we try to read an *opcode* outside the program, but this silently
                reads an *operand*. */
            }
            5 => {
                /*out */
                let operand = combo_operand(program, pc + 1, registers) % 8;
                outputs.push(operand.try_into().unwrap());
                pc += 2;
            }
            6 => {
                /*bdv*/
                let denom = 1 << combo_operand(program, pc + 1, registers);
                registers[1] = registers[0] / denom;
                pc += 2;
            }
            7 => {
                /*cdv*/
                let denom = 1 << combo_operand(program, pc + 1, registers);
                registers[2] = registers[0] / denom;
                pc += 2;
            }

            _ => {
                panic!("Invalid opcode {opcode} at pc {pc}");
            }
        }
    }
    outputs
}

fn main() -> std::io::Result<()> {
    /*    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');*/
    let mut initial_a = 1;
    let mut high_a;
    let mut low_a = 1;
    let program: Vec<i64> = vec![2, 4, 1, 1, 7, 5, 1, 4, 0, 3, 4, 5, 5, 5, 3, 0];

    /* This relies on two assumptions gathered from watching behaviour of my input, but may
    not apply to yours.
    1) Output becomes longer as the initial value of A increases
    2) Smaller changes in A affect the early output values, and larger changes affect the
       later output values.
    */

    // First, increase A until we get the same length output
    loop {
        let mut registers: Vec<i64> = vec![initial_a, 0, 0];
        let outputs = run(&program, &mut registers);
        if outputs.len() > program.len() {
            high_a = initial_a;
            break;
        }
        if outputs.len() < program.len() {
            low_a = initial_a;
        }
        initial_a *= 10;
    }
    println!("Range of initial values is between {low_a} and {high_a}");

    // Now gradually step down the increment value and increase the number of output values
    // checked at the end of the output, until we check them all.
    let mut match_numbers = program.len() - 1;
    loop {
        let increment = if (high_a - low_a) > 100 {
            (high_a - low_a) / 100
        } else {
            1
        };
        initial_a = low_a;
        let mut found_match: bool = false;
        loop {
            let mut registers: Vec<i64> = vec![initial_a, 0, 0];
            let outputs = run(&program, &mut registers);
            if outputs.len() > program.len() {
                break;
            }
            if outputs[match_numbers..] == program[match_numbers..] {
                found_match = true;
            } else if found_match {
                high_a = initial_a;
                break;
            } else {
                low_a = initial_a;
            }
            initial_a += increment;
        }
        println!("Range of initial values is between {low_a} and {high_a} (exclusive)");
        if match_numbers == 0 {
            break;
        }
        match_numbers -= 1;
    }
    Ok(())
}
