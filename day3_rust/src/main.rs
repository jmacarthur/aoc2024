use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

fn count_multiples(contents: &str, use_switch: bool) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do)(\(\))|(don't)(\(\))").unwrap();
    let mut fields = vec![];
    let mut active = true;
    for (_, [a, b]) in re.captures_iter(contents).map(|c| c.extract()) {
        match a {
            "do" => {
                active = true;
            }
            "don't" => {
                active = false;
            }
            _ => {
                if active || !use_switch {
                    fields.push((parse_field(a), parse_field(b)));
                }
            }
        }
    }

    let mut total = 0;
    for (a, b) in fields {
        total += a * b;
    }
    total
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input3.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Part 1 total: {}", count_multiples(&contents, false));
    println!("Part 2 total: {}", count_multiples(&contents, true));
    Ok(())
}
