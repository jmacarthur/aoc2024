use std::cmp::Ordering;
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

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut token_iterator = contents.split_whitespace();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    while let Some(s) = token_iterator.next() {
        list1.push(parse_field(s));
        match token_iterator.next() {
            Some(s) => {
                list2.push(parse_field(s));
            }
            None => {
                break;
            }
        }
    }

    assert!(list1.len() == list2.len());
    list1.sort();
    list2.sort();
    let mut total: i32 = 0;
    assert!(list1.len() == list2.len());

    let mut list2_iterator = list2.iter();
    for i1 in list1.iter() {
        let i2 = list2_iterator.next().unwrap();
        total += (i1 - i2).abs();
    }
    println!("Total difference (part 1) is {}", total);
    total = 0;
    let mut list2_iterator = list2.iter().peekable();
    'outer: for i1 in list1.iter() {
        let mut match_count = 0;
        loop {
            match list2_iterator.peek() {
                Some(&x) => match x.cmp(i1) {
                    Ordering::Less => list2_iterator.next(),
                    Ordering::Equal => {
                        match_count += 1;
                        list2_iterator.next()
                    }
                    Ordering::Greater => {
                        break;
                    }
                },
                None => break 'outer,
            };
        }
        total += i1 * match_count;
    }
    println!("Total for part 2: {}", total);

    Ok(())
}
