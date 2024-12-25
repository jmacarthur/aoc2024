use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

mod aoc_utils;
use crate::aoc_utils::*;

fn prune(start: u64) -> u64 {
    start % 16777216
}

fn mix(x: u64, y: u64) -> u64 {
    x ^ y
}

fn evolve(start: u64) -> u64 {
    let a = start * 64;
    let b = prune(mix(start, a));
    let c = prune(mix(b / 32, b));
    let d = prune(mix(c*2048, c));
    d
}

fn run_sales(target: &VecDeque::<i32>, starting_numbers: &Vec::<u64>) -> i32
{
    let mut total = 0u64;
    let mut bananas = 0i32;
    for s in starting_numbers {
        let mut evolved: u64 = *s;
        let mut last_price: i32 = (s % 10).try_into().unwrap();
        let mut diffs = VecDeque::<i32>::new();
        let mut matched_target = false;
        for _i in 0..2000 {
            evolved = evolve(evolved);
            let price: i32 = (evolved % 10).try_into().unwrap();
            let diff = price - last_price;
            diffs.push_back(diff);
            if diffs.len() > 4 {
                diffs.pop_front();
            }
            if diffs == *target && !matched_target {
                matched_target = true;
                bananas += price;
                break;
            }
            last_price = price;

        }
        total += evolved;
    }
    //println!("Total {total}");
    //println!("Total bananas {bananas}");
    bananas
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut starting_numbers = Vec::<u64>::new();
    for line in line_iterator {
        if !line.is_empty() {
            starting_numbers.push(parse_field(line).try_into().unwrap());
        }
    }
    println!("{:?}", starting_numbers);
    let mut best = 0;
    for a in -9..=9 {
        for b in -9..=9 {
            println!("Seq {a},{b}");
            for c in -9..=9 {
                for d in -9..=9 {
                    let range = a+b+c+d;
                    if range <= 9 && range >= -9 {
                        let target = VecDeque::from([a,b,c,d]);
                        let bananas = run_sales(&target, &starting_numbers);
                        if bananas > best {
                            println!("{:?} -> {bananas}, best so far", target);
                            best = bananas;
                        }
                    }
                }
            }
        }        
    }
    Ok(())
}
