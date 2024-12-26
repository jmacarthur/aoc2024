use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

fn add_device(locks: &mut Vec<Grid>, keys: &mut Vec<Grid>, rows: Vec<Vec<char>>) {
    assert!(rows.len() == 7);
    if rows[0] == ['#'; 5] {
        locks.push(Grid::new(rows));
    } else {
        keys.push(Grid::new(rows));
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');

    let mut locks = Vec::<Grid>::new();
    let mut keys = Vec::<Grid>::new();
    let mut current_rows = vec![];

    for line in line_iterator {
        if line.is_empty() {
            add_device(&mut locks, &mut keys, current_rows);
            current_rows = vec![];
        } else {
            current_rows.push(line.chars().collect());
        }
    }
    if !current_rows.is_empty() {
        add_device(&mut locks, &mut keys, current_rows);
    }
    println!("{} locks and {} keys collected.", locks.len(), keys.len());
    let mut fitting_combinations = 0;
    // It wouldn't be difficult to scan all these into pin & key height information,
    // making the comparison much quicker. But given the number of devices is low,
    // this implementation is fast enough and gives a more direct representation of what's
    // being checked.
    for lock in &locks {
        for key in &keys {
            let mut key_fits = true;
            'gridscan: for y in 0..key.height() {
                for x in 0..key.width() {
                    if key.get(x, y) == Some('#') && lock.get(x, y) == Some('#') {
                        key_fits = false;
                        break 'gridscan;
                    }
                }
            }
            if key_fits {
                fitting_combinations += 1;
            }
        }
    }
    println!(
        "{} unique pairs of keys and locks fit together.",
        fitting_combinations
    );
    Ok(())
}
