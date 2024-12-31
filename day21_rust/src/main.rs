mod aoc_utils;

mod day21_part1;
use crate::day21_part1::*;

mod day21_part2;
use crate::day21_part2::*;

fn main() {
    // Initial checks
    let l1 = find_route('A', '1', numeric_pad_positions);
    println!("Route from A to 1 on numeric pad is {:?}", l1);
    let l2 = find_route('A', '<', directional_pad_positions);
    println!("Route from A to < on directional pad is {:?}", l2);

    let input_sequences: Vec<&str> = Vec::from(["671A", "826A", "670A", "085A", "283A"]);

    let test_sequences: Vec<&str> = Vec::from(["029A", "980A", "179A", "456A", "379A"]);

    let sequences = input_sequences;

    println!("Part 1 total checksum is {}", solve_part1(&sequences));
    println!("Part 2 total checksum is {}", solve_part2(&sequences));
}
