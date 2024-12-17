use pathfinding::prelude::astar;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
    dir: i32,
}

static TURN_COST: u32 = 1000000;
static MOVE_COST: u32 = 1000;
static VISITED_COST: u32 = 1;

fn turn_cost(olddir: i32, newdir: i32) -> u32 {
    cmp::min((olddir + 4 - newdir) % 4, (newdir + 4 - olddir) % 4)
        .try_into()
        .unwrap()
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn successors(pos: &Pos, grid: &Grid) -> Vec<(Pos, u32)> {
    let &Pos { x, y, dir: d } = pos;
    let mut successors = vec![];
    let directions = [(1, 0, 0), (0, -1, 1), (-1, 0, 2), (0, 1, 3)];
    for (dx, dy, newdir) in directions {
        match grid.get(x + dx, y + dy) {
            Some('.') => {
                successors.push((
                    Pos {
                        x: x + dx,
                        y: y + dy,
                        dir: newdir,
                    },
                    MOVE_COST + TURN_COST * turn_cost(d, newdir),
                ));
            }
            Some('*') => {
                successors.push((
                    Pos {
                        x: x + dx,
                        y: y + dy,
                        dir: newdir,
                    },
                    MOVE_COST + TURN_COST * turn_cost(d, newdir) + VISITED_COST,
                ));
            }
            _ => {}
        }
    }
    successors
}

fn count_seats(grid: &Grid) -> i32 {
    let mut total_seats = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == Some('*') {
                total_seats += 1;
            }
        }
    }
    total_seats
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');

    let mut start_pos = Pos { x: 0, y: 0, dir: 0 };
    let mut end_pos = Pos { x: 0, y: 0, dir: 1 };
    let mut reading_grid = true;
    let mut directions = String::new();
    let mut rows = vec![];
    for (y, line) in line_iterator.enumerate() {
        if line.is_empty() {
            reading_grid = false;
        } else if reading_grid {
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
        } else {
            directions.push_str(line);
        }
    }
    println!(
        "Routing from {}, {} to {}, {}",
        start_pos.x, start_pos.y, end_pos.x, end_pos.y
    );
    let mut grid = Grid::new(rows);
    let mut worst_fractional = 0;
    loop {
        let result = astar(
            &start_pos,
            |p| successors(p, &grid),
            |p| p.distance(&end_pos),
            |p| p.x == end_pos.x && p.y == end_pos.y,
        );
        if let Some((moves, cost)) = result {
            for p in moves {
                grid.set(p.x, p.y, '*');
            }
            grid.dump();
            let remainder = cost % MOVE_COST;
            println!("Final cost: {} (rem {})", cost / MOVE_COST, remainder);
            if remainder > worst_fractional || remainder == 0 {
                worst_fractional = remainder;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("Seats along route: {}", count_seats(&grid));
    Ok(())
}
