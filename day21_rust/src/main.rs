use std::collections::VecDeque;

mod aoc_utils;
use crate::aoc_utils::*;

fn numeric_pad_positions(c: char) -> (i32, i32) {
    match c {
        '7' => (0,0),
        '8' => (1,0),
        '9' => (2,0),
        '4' => (0,1),
        '5' => (1,1),
        '6' => (2,1),
        '1' => (0,2),
        '2' => (1,2),
        '3' => (2,2),
        '0' => (1,3),
        'A' => (2,3),
        _ => { panic!("No character {c} on the numeric pad"); },
    }
}

fn directional_pad_positions(c: char) -> (i32, i32) {
    match c {
        '^' => (1,0),
        'A' => (2,0),
        '<' => (0,1),
        'v' => (1,1),
        '>' => (2,1),
        _ => { panic!("No character {c} on the directional pad"); },
    }
}

fn reverse_numeric(x: i32, y:i32) -> Option<char> {
    for c in "0123456789A".chars() {
        if numeric_pad_positions(c) == (x,y) {
            return Some(c);
        }
    }
    None
}

fn reverse_directional(x: i32, y:i32) -> Option<char> {
    for c in "^<>vA".chars() {
        if directional_pad_positions(c) == (x,y) {
            return Some(c);
        }
    }
    None
}



fn find_route(from: char, to: char, position: impl Fn(char) -> (i32, i32),reverse_fn: impl Fn(i32, i32) -> Option<char>) -> Vec<char> {
    let mut route = vec![];
    let (from_x, from_y) = position(from);

    let (to_x, to_y) = position(to);
    let dx = (to_x-from_x).signum();
    let dy = (to_y-from_y).signum();
    let mut x = from_x;
    let mut y = from_y;
    let mut vertical_moves = vec![];
    let mut horizontal_moves = vec![];
    let mut route_invalid = false;
    while x != to_x {
        if dx < 0 {
            horizontal_moves.push('<');
            x -= 1;
        } else {
            horizontal_moves.push('>');
            x += 1;
        }
        if reverse_fn(x, y) == None {
            route_invalid = true;
        }
    }
    while y != to_y {
        if dy < 0 {
            vertical_moves.push('^');
            y -= 1;
        } else {
            vertical_moves.push('v');
            y += 1;
        }
    }

    if route_invalid {
        route.extend(vertical_moves);
        route.extend(horizontal_moves);
    } else {
        route.extend(horizontal_moves);
        route.extend(vertical_moves);
    }
    route.push('A');
    route
}

fn check_route_directional(route: Vec<char>) -> Vec<char> {
    let (mut x, mut y) = directional_pad_positions('A');
    let mut result = vec![];
    for c in route {
        match c {
            '<' => {x -= 1;},
            '>' => {x += 1;},
            '^' => {y -= 1;},
            'v' => {y += 1;},
            'A' => { result.push(reverse_directional(x,y).unwrap());},
            _ => { panic!("Invalid char in sequence {c}"); }
        }
        if reverse_directional(x,y) == None {
            panic!("Robot hit invalid directional position {x}, {y}");
        }
    }
    result
}

fn check_route_numeric(route: Vec<char>) -> Vec<char> {
    let (mut x, mut y) = numeric_pad_positions('A');
    let mut result = vec![];
    for c in route {
        match c {
            '<' => {x -= 1;},
            '>' => {x += 1;},
            '^' => {y -= 1;},
            'v' => {y += 1;},
            'A' => { result.push(reverse_numeric(x,y).unwrap());
            println!("Valid press {c}");},
            _ => { panic!("Invalid char in sequence {c}"); }
        }
        if reverse_numeric(x,y) == None {
            panic!("Robot hit invalid numeric position {x}, {y}");
        }
    }
    result
}

fn find_sequence(sequence: &Vec<char>) -> Vec<char> {
    let mut route1 = vec![]; // Route on first directional pad
    route1.extend(find_route('A', sequence[0], numeric_pad_positions, reverse_numeric));
    for pair in sequence.windows(2) {
        route1.extend(find_route(pair[0], pair[1], numeric_pad_positions, reverse_numeric));
    }
    let mut previous_route = route1;
    let mut route: Vec<char> = vec![];
    for i in 0..2 {
        route = vec![]; // Route on second directional pad
        route.extend(find_route('A', previous_route[0], directional_pad_positions, reverse_directional));
        for pair in previous_route.windows(2) {
            route.extend(find_route(pair[0], pair[1], directional_pad_positions, reverse_directional));
        }
        previous_route = route.clone();
    }
    route
}

fn main() {
    let l1 = find_route('A', '1', numeric_pad_positions, reverse_numeric);
    println!("{:?}", l1);
    let l2 = find_route('A', '<', directional_pad_positions, reverse_directional);
    println!("{:?}", l2);
    
    let input_sequences: Vec<&str> = Vec::from([
        "671A",
        "826A",
        "670A",
        "085A",
        "283A"
    ]);

    let test_sequences: Vec<&str> = Vec::from([
        "029A",
        "980A",
        "179A",
        "456A",
        "379A"
    ]);
    
    let sequences = test_sequences;

    let mut total_checksum = 0;
    for s in sequences {
        let num: usize = parse_field(&s[0..3]).try_into().unwrap();
        let route = find_sequence(&s.chars().collect());
        total_checksum += route.len() * num;
        println!("{:?} -> {:?}, len {}, checksum {}", s, route, route.len(), route.len()*num);
        let validate1 = check_route_directional(check_route_directional(route));
        println!("Validate1: {:?}", validate1);
        println!("Validate2: {:?}", check_route_numeric(validate1));
    }
    println!("Total checksum {}", total_checksum);
}
