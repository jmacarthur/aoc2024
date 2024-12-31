use std::collections::HashMap;

use crate::aoc_utils::*;
use crate::day21_part1::*;

static PART2_STEPS: i32 = 25;
static PART2_CACHELEVEL: i32 = 10;

fn find_directional_sequence(sequence: &[char]) -> Vec<char> {
    if sequence[0] == 'A' {
        return vec!['A'];
    }
    let mut route: Vec<char> = find_route('A', sequence[0], directional_pad_positions);
    for pair in sequence.windows(2) {
        route.extend(find_route(pair[0], pair[1], directional_pad_positions));
    }
    route
}

pub fn build_expansion_map() -> HashMap<Vec<char>, Vec<char>> {
    let mut expansion_map = HashMap::<Vec<char>, Vec<char>>::new();
    for dx in -2..=2 {
        for dy in -3..=3 {
            let horizontal_directions = match dx {
                -2 => "<<",
                -1 => "<",
                0 => "",
                1 => ">",
                2 => ">>",
                _ => {
                    panic!();
                }
            };
            let vertical_directions = match dy {
                -3 => "^^^",
                -2 => "^^",
                -1 => "^",
                0 => "",
                1 => "v",
                2 => "vv",
                3 => "vvv",
                _ => {
                    panic!();
                }
            };
            let mut r1: Vec<char> = horizontal_directions
                .chars()
                .chain(vertical_directions.chars())
                .collect();
            r1.push('A');
            let r1expanded = find_directional_sequence(&r1);
            let mut r2: Vec<char> = vertical_directions
                .chars()
                .chain(horizontal_directions.chars())
                .collect();
            r2.push('A');
            let r2expanded = find_directional_sequence(&r2);
            expansion_map.insert(r1, r1expanded);
            expansion_map.insert(r2, r2expanded);
        }
    }
    expansion_map
}

pub fn expand(
    sequence: &[char],
    expansion_map: &HashMap<Vec<char>, Vec<char>>,
    result_cache: &HashMap<(&[char], i32), usize>,
    steps: i32,
) -> usize {
    if let Some(res) = result_cache.get(&(sequence, steps)) {
        return *res;
    }

    if let Some(expanded) = expansion_map.get(sequence) {
        if steps <= 1 {
            return expanded.len();
        }
        let splits = expanded.split_inclusive(|c| *c == 'A');
        let mut total_length = 0;
        for s in splits {
            total_length += expand(s, expansion_map, result_cache, steps - 1);
        }
        total_length
    } else {
        panic!("{:?} not in expansion map", sequence);
    }
}

pub fn solve_part2(sequences: &Vec<&str>) -> usize {
    // Part 2
    let expansion_map = build_expansion_map();

    let mut results_cache = HashMap::<(&[char], i32), usize>::new();
    for inp in expansion_map.keys() {
        for i in 1..PART2_CACHELEVEL {
            results_cache.insert((inp, i), expand(inp, &expansion_map, &results_cache, i));
        }
    }

    let mut total_checksum = 0;
    for s in sequences {
        let num: usize = parse_field(&s[0..3]).try_into().unwrap();
        let sequence: Vec<char> = s.chars().collect();
        let mut route1 = vec![]; // Route on first, numeric pad
        route1.extend(find_route('A', sequence[0], numeric_pad_positions));
        for pair in sequence.windows(2) {
            route1.extend(find_route(pair[0], pair[1], numeric_pad_positions));
        }

        let splits = route1.split_inclusive(|c| *c == 'A');
        let mut total_length = 0;
        for s in splits {
            total_length += expand(s, &expansion_map, &results_cache, PART2_STEPS);
        }

        println!("Test expansion of {:?}: {total_length}", sequence);
        total_checksum += total_length * num;
    }
    total_checksum
}
