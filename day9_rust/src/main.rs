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

fn simple_defrag(disk: &mut Vec<i32>) -> bool {
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

fn find_file(disk: &Vec<i32>, fileid: i32) -> (usize, usize) {
    let mut file_length = 0;
    let mut file_end_pos: Option<usize> = None;
    for (pos, n) in disk.iter().enumerate() {
        if *n == fileid {
            file_length += 1;
            file_end_pos = Some(pos);
        }
    }
    if let Some(p) = file_end_pos {
        (p+1-file_length, file_length)
    } else {
       panic!("No highest position found?");
    }
}


fn find_highest_file(disk: &Vec<i32>) -> (i32) {
    let mut highest = -1;
    for (pos, n) in disk.iter().enumerate() {
        if *n > highest {
            highest = *n;
        }
    }
    highest
}

fn find_free_space(disk: &Vec<i32>, length: usize) -> Option<usize> {
    let mut space_length: usize = 0;
    for (pos, n) in disk.iter().enumerate() {
        if *n >= 0 {
            space_length = 0;
        } else {
            space_length += 1;
            if space_length >= length {
                return Some(pos-length+1);
            }
        }
    }
    None
}

fn defrag_whole_file(disk: &mut Vec<i32>, fileid: i32) -> bool {
    let (pos, length) = find_file(disk, fileid);
    if let Some(free_space_pos) = find_free_space(disk, length) {
        println!("First free space of length {} starts at {}", length, free_space_pos);
        if free_space_pos < pos {
            for i in 0..length {
                disk[free_space_pos+i] = disk[pos+i];
                disk[pos+i] = -1;
            }
        }
        true
    } else {
        println!("No free space of length {}", length);
        false
    }
}

fn advanced_defrag(disk: &mut Vec<i32>) {
    let highest_file = find_highest_file(disk);

    for f in (0..=highest_file).rev() {
        println!("Defrag {}", f);
        defrag_whole_file(disk, f);
    }
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
    let part1: bool = false;
    if part1 {
        while (!simple_defrag(&mut blocks)) {};
        println!("{:?}", blocks);
        println!("{}", checksum(&blocks));
    } else {
        advanced_defrag(&mut blocks);
        println!("{:?}", blocks);
        println!("{}", checksum(&blocks));
    }
    Ok(())
}
