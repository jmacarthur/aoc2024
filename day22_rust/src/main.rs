use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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
    prune(mix(c * 2048, c))
}

fn run_sales(target: &VecDeque<i32>, starting_numbers: &Vec<u64>) -> i32 {
    let mut bananas = 0i32;
    for s in starting_numbers {
        let mut evolved: u64 = *s;
        let mut last_price: i32 = (s % 10).try_into().unwrap();
        let mut diffs = VecDeque::<i32>::new();
        for _i in 0..2000 {
            evolved = evolve(evolved);
            let price: i32 = (evolved % 10).try_into().unwrap();
            let diff = price - last_price;
            diffs.push_back(diff);
            if diffs.len() > 4 {
                diffs.pop_front();
            }
            if diffs == *target {
                bananas += price;
                break;
            }
            last_price = price;
        }
    }
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

    let mut total = 0u64;
    for s in &starting_numbers {
        let mut evolved: u64 = *s;
        for _i in 0..2000 {
            evolved = evolve(evolved);
        }
        total += evolved;
    }
    println!("Total for part 1: {total}");

    let mut best = 0;
    // I'm just going to brute force this but we can trim a few paths off:
    // For example, we can't have a sequence starting [-9,-5,...] or
    // [-4,-4,-4,...]
    for a in -9..=9 {
        for b in cmp::max(-9 - a, -9)..=cmp::min(9 - a, 9) {
            println!("Starting sequence with {a},{b}...");
            for c in -9.max(-9 - b).max(-9 - a - b)..=9.min(9 - a).min(9 - a - b) {
                for d in -9.max(-9 - a).max(-9 - a - b).max(-9 - a - b - c)
                    ..=9.min(9 - a).min(9 - a - b).min(9 - a - b - c)
                {
                    let target = VecDeque::from([a, b, c, d]);
                    let bananas = run_sales(&target, &starting_numbers);
                    if bananas > best {
                        println!("{:?} -> {bananas}, best so far", target);
                        best = bananas;
                    }
                }
            }
        }
    }
    println!("Best number of bananas is {best}");
    Ok(())
}
