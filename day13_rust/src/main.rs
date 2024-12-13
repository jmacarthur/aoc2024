use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

type Point = (i32, i32);

fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

struct Machine {
    a_inc_x: i32,
    a_inc_y: i32,
    b_inc_x: i32,
    b_inc_y: i32,
    target_x: i32,
    target_y: i32
}

impl Machine {
    fn new() -> Machine {
        Machine { a_inc_x: 0, a_inc_y: 0, b_inc_x: 0, b_inc_y: 0, target_x: 0, target_y: 0 }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut machine = Machine::new();
    let mut machines = Vec::<Machine>::new();

    let buttona_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let buttonb_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for line in line_iterator {
        if line.is_empty() {
            if machine.a_inc_x > 0 {
                machines.push(machine);
            }
            machine = Machine::new();
        } else {
            if let Some(captures) = buttona_regex.captures(line) {
                machine.a_inc_x = parse_field(&captures[1]);
                machine.a_inc_y = parse_field(&captures[2]);
            } else if let Some(captures) = buttonb_regex.captures(line) {
                machine.a_inc_x = parse_field(&captures[1]);
                machine.a_inc_y = parse_field(&captures[2]);
            } else if let Some(captures) = prize_regex.captures(line) {
                machine.target_x = parse_field(&captures[1]);
                machine.target_y = parse_field(&captures[2]);
            } else {
                panic!("Unmatched line: {}", line);
            }
        }
    }
    if machine.a_inc_x > 0 {
        machines.push(machine);
    }
    println!("{} machines read", machines.len());
    Ok(())
}
