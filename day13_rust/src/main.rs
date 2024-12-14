use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

struct Vector {
    x: i64,
    y: i64
}

struct Machine {
    a: Vector,
    b: Vector,
    target: Vector,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            a: Vector { x: 0, y: 0 },
            b: Vector { x: 0, y: 0 },
            target: Vector { x: 0, y: 0 }
        }
    }
}

fn find_ratios(a: &Vector, b: &Vector, target: &Vector) -> Option<(i64, i64)> {
    let numerator = target.x * a.y - target.y * a.x;
    let divisor = b.x * a.y - b.y * a.x;
    if divisor == 0 {
        println!("Zero divisor");
        None
    } else if numerator % divisor == 0 {
        let b_presses = numerator / divisor;
        let a_presses = (target.x - (b_presses * b.x)) / a.x;
        println!(
            "Found solution: ({}/{}) = {}/{}",
            numerator, divisor, a_presses, b_presses
        );
        if (a.x * a_presses + b.x * b_presses == target.x)
            && (a.y * a_presses + b.y * b_presses == target.y)
        {
            Some((a_presses, b_presses))
        } else {
            None
        }
    } else {
        println!(
            "Can't reach target with whole button presses ({}/{})",
            numerator, divisor
        );
        None
    }
}

fn main() -> std::io::Result<()> {
    let offset: i64 = 10000000000000; // Change to 0 for part 1.
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
            if machine.a.x > 0 {
                machines.push(machine);
            }
            machine = Machine::new();
        } else if let Some(captures) = buttona_regex.captures(line) {
            machine.a.x = parse_field(&captures[1]);
            machine.a.y = parse_field(&captures[2]);
        } else if let Some(captures) = buttonb_regex.captures(line) {
            machine.b.x = parse_field(&captures[1]);
            machine.b.y = parse_field(&captures[2]);
        } else if let Some(captures) = prize_regex.captures(line) {
            machine.target.x = parse_field(&captures[1]) + offset;
            machine.target.y = parse_field(&captures[2]) + offset;
        } else {
            panic!("Unmatched line: {}", line);
        }
    }
    if machine.a.x > 0 {
        machines.push(machine);
    }

    println!("{} machines read", machines.len());

    let mut spend = 0;
    for m in machines {
        let mut score1 = None;
        let mut score2 = None;
        println!(
            "Solving {} {} {} {} -> {} {}",
            m.a.x, m.a.y, m.b.x, m.b.y, m.target.x, m.target.y
        );
        if let Some((a, b)) = find_ratios(&m.a, &m.b, &m.target) {
            println!("A: {a}, B: {b}");
            score1 = Some(3 * a + b);
        }
        if let Some((b, a)) = find_ratios(&m.b, &m.a, &m.target) {
            println!("A: {a}, B: {b}");
            score2 = Some(3 * a + b);
        }
        match (score1, score2) {
            (None, None) => {}
            (Some(s), None) => {
                spend += s;
                println!("One solution, by method a");
            }
            (None, Some(s)) => {
                spend += s;
                println!("One solution, by method b");
            }
            (Some(s1), Some(s2)) => {
                println!("Two solutions found, with scores {s1}, {s2}");
                if s1 < s2 {
                    spend += s1;
                } else {
                    spend += s2;
                }
            }
        };
    }
    println!("Total spend {spend}");
    Ok(())
}
