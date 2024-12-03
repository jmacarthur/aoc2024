use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}
fn check_reactor(values: &[i32]) -> bool {
    let mut value_iterator = values.iter().peekable();
    let mut valid = true;
    let mut direction = 0;
    while let Some(value) = value_iterator.next() {
        match value_iterator.peek() {
            Some(&nextval) => {
                let diff = value - nextval;
                if diff.abs() > 3 || diff == 0 {
                    valid = false;
                }
                if direction != 0 && diff.signum() != direction {
                    valid = false;
                }
                direction = diff.signum();
            }
            None => {
                break;
            }
        }
    }
    valid
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut reactor_lines = Vec::new();
    for line in line_iterator {
        let mut values = Vec::new();
        let value_iterator = line.split_whitespace();

        for v in value_iterator {
            values.push(parse_field(v));
        }
        if !values.is_empty() {
            reactor_lines.push(values);
        }
    }

    let reactor_iterator = reactor_lines.iter();
    let mut valid_count = 0;
    for values in reactor_iterator {
        let valid = check_reactor(values);
        if valid {
            valid_count += 1;
        }
    }
    println!("Total valid reactors for part 1: {}", valid_count);

    let reactor_iterator = reactor_lines.iter();
    let mut valid_count = 0;
    for values in reactor_iterator {
        for i in 0..values.len() {
            let slice = [&values[..i], &values[i + 1..]].concat();
            let valid = check_reactor(&slice);
            if valid {
                valid_count += 1;
                break;
            }
        }
    }
    println!("Total valid reactors for part 2: {}", valid_count);

    Ok(())
}
