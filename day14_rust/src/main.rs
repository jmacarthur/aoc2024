use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

struct Vector {
    x: i32,
    y: i32,
}

struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Robot {
        Robot {
            position: Vector { x, y },
            velocity: Vector { x: dx, y: dy },
        }
    }
}

fn assemble_grid(robots: &Vec<Robot>, width: i32, height: i32) -> Grid {
    let mut rows = Vec::<Vec<char>>::new();
    for _ in 0..height {
        let row = vec!['.'; width.try_into().unwrap()];
        rows.push(row);
    }
    let mut grid = Grid::new(rows);
    for r in robots {
        grid.set(r.position.x, r.position.y, '#');
    }
    grid
}

fn count_continuous(robots: &Vec<Robot>, width: i32, height: i32) -> usize {
    // Count the number of robots with no gap in any single line; return the
    // maximum number of continuous robots.
    let mut max_continuous = 0;
    let grid = assemble_grid(robots, width, height);
    for y in 0..height {
        let mut continuous_robots = 0;
        for x in 0..height {
            if grid.get(x, y) == Some('#') {
                continuous_robots += 1;
                if continuous_robots > max_continuous {
                    max_continuous = continuous_robots;
                }
            } else {
                continuous_robots = 0;
            }
        }
    }
    max_continuous
}

fn dump(robots: &Vec<Robot>, width: i32, height: i32) {
    let grid = assemble_grid(robots, width, height);
    for y in 0..height {
        let mut line: String = String::new();
        for x in 0..width {
            if let Some(c) = grid.get(x, y) {
                line.push(c);
            } else {
                panic!("Went outside grid during dump? at {}, {}", x, y);
            }
        }
        println!("{}", line);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut robots = Vec::<Robot>::new();

    let robot_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in line_iterator {
        if !line.is_empty() {
            if let Some(captures) = robot_regex.captures(line) {
                let robot = Robot::new(
                    parse_field(&captures[1]),
                    parse_field(&captures[2]),
                    parse_field(&captures[3]),
                    parse_field(&captures[4]),
                );
                robots.push(robot);
            } else {
                panic!("Unmatched line: {}", line);
            }
        }
    }

    let width = 101;
    let height = 103;
    let mid_x = width / 2;
    let mid_y = height / 2;
    let max_cycles = 1000000; // Change to 100 for part 1
    println!(
        "Quadrants are 0-{} and {} to {}",
        mid_x - 1,
        mid_x + 1,
        width - 1
    );
    println!("{} robots read", robots.len());
    let mut max_continuous = 0;
    for cycle in 0..max_cycles {
        for r in &mut robots {
            r.position.x = (r.position.x + r.velocity.x).rem_euclid(width);
            r.position.y = (r.position.y + r.velocity.y).rem_euclid(height);
        }
        let c = count_continuous(&robots, width, height);
        if c > max_continuous {
            max_continuous = c;
            println!("After {} seconds, continuous count is {c}:", cycle + 1);
            dump(&robots, width, height);
        }
    }

    // Count quadrants for part 1
    let mut quad_count = [0, 0, 0, 0];

    for r in robots {
        if r.position.x < mid_x && r.position.y < mid_y {
            quad_count[0] += 1;
        } else if r.position.x > mid_x && r.position.y < mid_y {
            quad_count[1] += 1;
        } else if r.position.x < mid_x - 1 && r.position.y > mid_y {
            quad_count[2] += 1;
        } else if r.position.x > mid_x && r.position.y > mid_y {
            quad_count[3] += 1;
        }
    }
    println!("Quads: {:?}", quad_count);
    println!(
        "Score: {}",
        quad_count[0] * quad_count[1] * quad_count[2] * quad_count[3]
    );
    Ok(())
}
