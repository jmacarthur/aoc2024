use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_field(text: &str) -> i64 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

struct Machine {
    a_inc_x: i64,
    a_inc_y: i64,
    b_inc_x: i64,
    b_inc_y: i64,
    target_x: i64,
    target_y: i64,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            a_inc_x: 0,
            a_inc_y: 0,
            b_inc_x: 0,
            b_inc_y: 0,
            target_x: 0,
            target_y: 0,
        }
    }
}

fn find_ratios(m: &Machine) -> Option<(i64, i64)> {
    let numerator = m.target_x * m.a_inc_y - m.target_y * m.a_inc_x;
    let divisor = m.b_inc_x * m.a_inc_y - m.b_inc_y * m.a_inc_x;
    if divisor == 0 {
        println!("Zero divisor");
        None
    } else if numerator % divisor == 0 {
        let b = numerator / divisor;
        /*if (m.target_x - (b*m.a_inc_x)) % m.a_inc_x == 0 {
            return None
        }*/
        let a = (m.target_x - (b * m.b_inc_x)) / m.a_inc_x;
        println!(
            "Found solution via method a: ({}/{}) = {}*A {}*B",
            numerator, divisor, a, b
        );
        if (m.a_inc_x * a + m.b_inc_x * b == m.target_x)
            && (m.a_inc_y * a + m.b_inc_y * b == m.target_y)
        {
            Some((a, b))
        } else {
            None
        }
    } else {
        println!(
            "(a) Can't reach target with whole button presses ({}/{})",
            numerator, divisor
        );
        None
    }
}

fn find_ratios_b(m: &Machine) -> Option<(i64, i64)> {
    let numerator = m.target_x * m.b_inc_y - m.target_y * m.b_inc_x;
    let divisor = m.a_inc_x * m.b_inc_y - m.a_inc_y * m.b_inc_x;
    if divisor == 0 {
        println!("Zero divisor");
        None
    } else if numerator % divisor == 0 {
        let a = numerator / divisor;
        /*if (m.target_x - (a*m.a_inc_x)) % m.b_inc_x == 0 {
            return None
        }*/
        let b = (m.target_x - (a * m.a_inc_x)) / m.b_inc_x;
        println!(
            "Found solution via method b: ({}/{}) = {}*A {}*B",
            numerator, divisor, a, b
        );
        if (m.a_inc_x * a + m.b_inc_x * b == m.target_x)
            && (m.a_inc_y * a + m.b_inc_y * b == m.target_y)
        {
            Some((a, b))
        } else {
            None
        }
    } else {
        println!(
            "(b) Can't reach target with whole button presses ({}/{})",
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
            if machine.a_inc_x > 0 {
                machines.push(machine);
            }
            machine = Machine::new();
        } else if let Some(captures) = buttona_regex.captures(line) {
            machine.a_inc_x = parse_field(&captures[1]);
            machine.a_inc_y = parse_field(&captures[2]);
        } else if let Some(captures) = buttonb_regex.captures(line) {
            machine.b_inc_x = parse_field(&captures[1]);
            machine.b_inc_y = parse_field(&captures[2]);
        } else if let Some(captures) = prize_regex.captures(line) {
            machine.target_x = parse_field(&captures[1]) + offset;
            machine.target_y = parse_field(&captures[2]) + offset;
        } else {
            panic!("Unmatched line: {}", line);
        }
    }
    if machine.a_inc_x > 0 {
        machines.push(machine);
    }

    println!("{} machines read", machines.len());

    let mut spend = 0;
    for m in machines {
        let mut score1 = None;
        let mut score2 = None;
        println!(
            "Solving {} {} {} {} -> {} {}",
            m.a_inc_x, m.a_inc_y, m.b_inc_x, m.b_inc_y, m.target_x, m.target_y
        );
        if let Some((a, b)) = find_ratios(&m) {
            println!("A: {a}, B: {b}");
            score1 = Some(3 * a + b);
        }
        if let Some((a, b)) = find_ratios_b(&m) {
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
