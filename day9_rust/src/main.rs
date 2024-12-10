use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn char_to_int(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => -1,
    }
}

fn last_real_block_pos(disk: &Vec<i32>) -> Option<usize> {
    let mut pos: Option<usize> = None;
    for (position, fileid) in (*disk).iter().enumerate() {
        if *fileid >= 0 {
            pos = Some(position);
        }
    }
    pos
}

fn first_free_space_pos(disk: &Vec<i32>) -> Option<usize> {
    for (position, fileid) in (*disk).iter().enumerate() {
        if *fileid < 0 {
            let pos = position.try_into().expect("Negative position from enumerate?");
            return Some(pos);
        }
    }
    None
}

fn defrag(disk: &mut Vec<i32>) -> bool {
    if let Some(i) = last_real_block_pos(disk) {
        if let Some(j) = first_free_space_pos(disk) {
            if i < j {
                println!("Finished!");
                return true;
            }
            disk[j] = disk[i];
            disk[i] = -1;
        
        } else {
            panic!("No free spaces on disk");
        }
        
    } else {
        panic!("No real blocks on disk");
    }
    false
}

fn checksum(disk: &Vec<i32>) -> i64 {
    let mut total = 0i64;
    for (pos, fileid) in disk.iter().enumerate() {
        if *fileid >= 0 {
            let fnum: i64 = TryInto::<i64>::try_into(*fileid).unwrap();
            total += TryInto::<i64>::try_into(pos).unwrap() * fnum;
        }
    }
    total
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input9.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut blocks: Vec<i32> = vec![];
    let mut file_index = 0i32;
    for (inputpos, char) in contents.chars().enumerate() {
        let field_size = char_to_int(char);
        if field_size >= 0 {
            if inputpos % 2 == 0 {
                // It's a file
                for _i in 0..char_to_int(char) {
                    blocks.push(file_index);
                }
                file_index += 1;
            } else {
                // It's free space
                for _i in 0..char_to_int(char) {
                    blocks.push(-1);
                }
            }
        }
    }
    println!("{:?}", blocks);
    
    while (!defrag(&mut blocks)) {};

    println!("{:?}", blocks);
    println!("{}", checksum(&blocks));
    Ok(())
}
