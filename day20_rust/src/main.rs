use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn successors(pos: &Pos, grid: &Grid) -> Vec<(Pos, u32)> {
    let &Pos { x, y } = pos;
    let mut successors = vec![];
    for (dx, dy) in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
        if grid.get(x + dx, y + dy) == Some('.') {
            successors.push((
                Pos {
                    x: x + dx,
                    y: y + dy,
                },
                1,
            ));
        }
    }
    successors
}

fn reachable_positions(x: i32, y: i32, range: i32) -> Vec<Pos> {
    let mut destinations = vec![];
    for d in 0..range {
        destinations.push(Pos {
            x: x + d,
            y: y + range - d,
        });
        destinations.push(Pos {
            x: x - d,
            y: y - (range - d),
        });
        destinations.push(Pos {
            x: x - (range - d),
            y: y + d,
        });
        destinations.push(Pos {
            x: x + (range - d),
            y: y - d,
        });
    }
    destinations
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');

    let mut start_pos = Pos { x: 0, y: 0 };
    let mut end_pos = Pos { x: 0, y: 0 };

    let mut rows = vec![];

    for (y, line) in line_iterator.enumerate() {
        if !line.is_empty() {
            let mut row: Vec<char> = vec![];
            for (x, char) in line.chars().enumerate() {
                if char == 'S' {
                    start_pos.x = x.try_into().unwrap();
                    start_pos.y = y.try_into().unwrap();
                    row.push('.');
                } else if char == 'E' {
                    end_pos.x = x.try_into().unwrap();
                    end_pos.y = y.try_into().unwrap();
                    row.push('.');
                } else {
                    row.push(char);
                }
            }
            rows.push(row);
        }
    }

    let grid = Grid::new(rows);

    grid.dump();
    let original_path_length;

    if let Some(result) = astar(
        &start_pos,
        |p| successors(p, &grid),
        |p| p.distance(&end_pos),
        |p| p.x == end_pos.x && p.y == end_pos.y,
    ) {
        println!("Original path length is {}.", result.1);
        original_path_length = result.1;
    } else {
        panic!("No route found in initial map!");
    }

    let mut distance_from_start = HashMap::<Pos, u32>::new();
    let mut distance_to_end = HashMap::<Pos, u32>::new();
    for y in 0..grid.height() {
        println!("Computing distances ({}/{})", y, grid.height() - 1);
        for x in 0..grid.width() {
            if grid.get(x, y) == Some('.') {
                if let Some(result) = astar(
                    &start_pos,
                    |p| successors(p, &grid),
                    |p| p.distance(&Pos { x, y }),
                    |p| p.x == x && p.y == y,
                ) {
                    let new_path_length = result.1;
                    distance_from_start.insert(Pos { x, y }, new_path_length);
                }
                if let Some(result) = astar(
                    &Pos { x, y },
                    |p| successors(p, &grid),
                    |p| p.distance(&end_pos),
                    |p| p.x == end_pos.x && p.y == end_pos.y,
                ) {
                    let new_path_length = result.1;
                    distance_to_end.insert(Pos { x, y }, new_path_length);
                }
            }
        }
    }

    assert_eq!(Some(&original_path_length), distance_to_end.get(&start_pos));
    assert_eq!(
        Some(&original_path_length),
        distance_from_start.get(&end_pos)
    );

    let teleport_max_range = 20; // Reduce this to 2 for part 1
    let mut over_100 = 0;
    let mut savings = HashMap::<u32, u32>::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(ds) = distance_from_start.get(&Pos { x, y }) {
                for teleport_range in 1..=teleport_max_range {
                    let destinations = reachable_positions(x, y, teleport_range);
                    for destination in destinations {
                        if let Some(de) = distance_to_end.get(&destination) {
                            let actual_distance =
                                ds + de + <i32 as TryInto<u32>>::try_into(teleport_range).unwrap();
                            if actual_distance < original_path_length {
                                let saving = original_path_length - actual_distance;
                                if saving > 0 {
                                    if let Some(x) = savings.get(&saving) {
                                        savings.insert(saving, x + 1);
                                    } else {
                                        savings.insert(saving, 1);
                                    }
                                    if saving >= 100 {
                                        over_100 += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // To help debug, dump the number
    for s in savings.keys().sorted() {
        println!("Cheats saving {s}: {}", savings.get(s).unwrap());
    }
    println!("Number of cheats saving >=100ps: {}", over_100);
    Ok(())
}
