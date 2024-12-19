use pathfinding::prelude::astar;
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

static GRIDSIZE: i32 = 71;

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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');

    let start_pos = Pos { x: 0, y: 0 };
    let end_pos = Pos {
        x: GRIDSIZE - 1,
        y: GRIDSIZE - 1,
    };

    let mut bytes = Vec::<Pos>::new();
    let mut rows = vec![];
    for _ in 0..GRIDSIZE {
        rows.push(vec!['.'; GRIDSIZE.try_into().unwrap()]);
    }
    for line in line_iterator {
        if !line.is_empty() {
            let mut fields = line.split(",");
            bytes.push(Pos {
                x: parse_field(fields.next().unwrap()),
                y: parse_field(fields.next().unwrap()),
            });
        }
    }
    let mut grid = Grid::new(rows);

    for Pos { x, y } in &bytes[0..1024] {
        grid.set(*x, *y, '#');
    }
    grid.dump();
    if let Some(result) = astar(
        &start_pos,
        |p| successors(p, &grid),
        |p| p.distance(&end_pos),
        |p| p.x == end_pos.x && p.y == end_pos.y,
    ) {
        println!(
            "After dropping 1024 bytes, the path length is {}.",
            result.1
        );
    } else {
        panic!("No route found after dropping 1024 bytes?");
    }

    let min_drops = 1024;
    let max_drops = bytes.len();
    let mut lowest_passing_value = min_drops;
    let mut highest_failing_value = max_drops;

    let mut test_grid = grid.clone();
    loop {
        let adjust = (highest_failing_value - lowest_passing_value) / 2;
        let test_value = lowest_passing_value + adjust;
        for Pos { x, y } in &bytes[min_drops..=test_value] {
            test_grid.set(*x, *y, '#');
        }
        if let Some(result) = astar(
            &start_pos,
            |p| successors(p, &test_grid),
            |p| p.distance(&end_pos),
            |p| p.x == end_pos.x && p.y == end_pos.y,
        ) {
            println!("{}: Route succeeded at length {}", test_value, result.1);
            lowest_passing_value = test_value;
        } else {
            println!("{}: Route failed", test_value);
            highest_failing_value = test_value;
            test_grid = grid.clone();
            // TODO: We don't need to start from min_drops again in this case!
        }
        if adjust <= 1 {
            break;
        }
    }
    println!(
        "The first byte which blocks the route is no. {}: {:?}",
        highest_failing_value, bytes[highest_failing_value]
    );

    Ok(())
}
