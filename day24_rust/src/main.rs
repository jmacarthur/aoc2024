use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

mod aoc_utils;
use crate::aoc_utils::*;

type WireName = String;

static HIGHEST_BIT: i64 = 45;

#[derive(Debug, PartialEq)]
enum Function {
    And,
    Or,
    Xor,
}

impl FromStr for Function {
    type Err = ();

    fn from_str(input: &str) -> Result<Function, Self::Err> {
        match input {
            "AND" => Ok(Function::And),
            "OR" => Ok(Function::Or),
            "XOR" => Ok(Function::Xor),
            _ => Err(()),
        }
    }
}

struct Gate {
    input1: WireName,
    input2: WireName,
    output: WireName,
    function: Function,
}

impl Gate {
    fn calc(&self, input1: u8, input2: u8) -> u8 {
        match self.function {
            Function::And => input1 & input2,
            Function::Or => input1 | input2,
            Function::Xor => input1 ^ input2,
        }
    }
    fn function_name(&self) -> &str {
        match self.function {
            Function::And => "&",
            Function::Or => "|",
            Function::Xor => "^",
        }
    }
}

fn get_variable(wires: &HashMap<WireName, u8>, variable_name: &str) -> u64 {
    // Returns a variable ('x','y', or 'z') by looking for all the wires named
    // x00, x01, x02.. for example and populating a 64-bit value.
    let mut variable: u64 = 0;
    for (wirename, value) in wires {
        if wirename.starts_with(variable_name) && *value == 1 {
            let power: u64 = parse_field(&wirename[1..]).try_into().unwrap();
            let setbit: u64 = 1u64 << power;
            variable |= setbit;
        }
    }
    variable
}

#[allow(unused)]
fn get_bin_variable(wires: &HashMap<WireName, u8>, variable_name: &str) -> String {
    // As get_variable, but returns a 64-bit string version of the variable, to help
    // debugging.
    let mut checksum: [char; 64] = ['0'; 64];
    for (wirename, value) in wires {
        if wirename.starts_with(variable_name) && *value == 1 {
            let power: usize = parse_field(&wirename[1..]).try_into().unwrap();
            checksum[63 - power] = '1';
        }
    }
    checksum.iter().collect()
}

#[allow(unused)]
fn dump_route(gates: &Vec<Gate>, target: String) -> String {
    if target.starts_with('x') || target.starts_with('y') {
        return target;
    }
    for gate in gates {
        if gate.output == *target {
            return format!(
                "({}{}{})",
                dump_route(gates, gate.input1.clone()),
                gate.function_name(),
                dump_route(gates, gate.input2.clone())
            );
        }
    }
    "INVALID".to_string()
}

#[allow(unused)]
fn set_input(wires: &mut HashMap<WireName, u8>, varname: String, value: u64) {
    for i in 0..HIGHEST_BIT {
        let wirename = format!("{}{:02}", varname, i);
        println!("Set {}", wirename);
        let bit: u8 = ((value >> i) & 1).try_into().unwrap();
        wires.insert(wirename, bit);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut wires = HashMap::<WireName, u8>::new();
    let mut input_gates = false;
    let wire_regex = Regex::new(r"(.{3}): (\d)").unwrap();
    let gate_regex = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").unwrap();

    let mut gates = vec![];
    for line in line_iterator {
        if line.is_empty() {
            input_gates = true;
        } else if input_gates {
            if let Some(captures) = gate_regex.captures(line) {
                println!("{}", line);
                let gate = Gate {
                    input1: captures[1].to_string(),
                    input2: captures[3].to_string(),
                    output: captures[4].to_string(),
                    function: Function::from_str(&captures[2]).unwrap(),
                };
                gates.push(gate);
            }
        } else if let Some(captures) = wire_regex.captures(line) {
            let wirename = captures[1].to_string();
            wires.insert(wirename, parse_field(&captures[2]).try_into().unwrap());
        }
    }

    // The following code can be aded to generate random initial values
    // of x and y, for testing part 2.
    /* use rand::prelude::*;
    let mut rng = rand::thread_rng();
    set_input(&mut wires, "x".to_string(), rng.gen::<u64>() & ((1<<(HIGHEST_BIT+1))-1));
    set_input(&mut wires, "y".to_string(), rng.gen::<u64>() & ((1<<(HIGHEST_BIT+1))-1));
    */

    loop {
        let mut calculated_one_gate = false;
        for gate in &gates {
            if wires.contains_key(&gate.input1)
                && wires.contains_key(&gate.input2)
                && !wires.contains_key(&gate.output)
            {
                let result = gate.calc(wires[&gate.input1], wires[&gate.input2]);
                println!("Calculated {} = {}", gate.output, result);
                wires.insert(gate.output.clone(), result);
                calculated_one_gate = true;
            }
        }
        if !calculated_one_gate {
            break;
        }
    }

    let checksum = get_variable(&wires, "z");
    println!("Part 1: Checksum is {checksum}");
    let x = get_variable(&wires, "x");
    let y = get_variable(&wires, "y");
    let z = get_variable(&wires, "z");

    println!("Value of x is {} or {:#050b}", x, x);
    println!("Value of y is {} or {:#050b}", y, y);
    println!("Value of z is {} or {:#050b}", z, z);

    if x + y != z {
        println!("Variables mismatch");
    }

    Ok(())
}
