use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

struct Vector {
    x: i32,
    y: i32,
}

fn actually_move(grid: &mut Grid, pos: &Vector, dx: i32, dy: i32) -> bool {
    let (target_x, target_y) = (pos.x + dx, pos.y + dy);
    match grid.get(target_x, target_y) {
        Some('.') => {
            grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
            grid.set(pos.x, pos.y, '.');
            return true;
        },
        Some('O') => {
            if actually_move(grid, &Vector{x: target_x, y: target_y}, dx, dy) {
                grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                grid.set(pos.x, pos.y, '.');
                return true;
            } else {
                return false;
            }
        }
        _ => { return false; }
    };
    false
}

fn test_move(grid: &mut Grid, pos: &Vector, dx: i32, dy: i32) -> bool {
    let (target_x, target_y) = (pos.x + dx, pos.y + dy);
    match grid.get(target_x, target_y) {
        Some('.') => {
            return true;
        },
        Some('O') => {
            if test_move(grid, &Vector{x: target_x, y: target_y}, dx, dy) {
                return true;
            } else {
                return false;
            }
        }
        _ => { return false; }
    };
}


fn gps(grid: &Grid) -> i32 {
    let mut total = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x,y) == Some('O') {
                total += 100*y+x;
            }
        }
    }
    total
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    
    let mut robot_pos = Vector{x: 0, y:0};

    let mut reading_grid = true;
    let mut directions = String::new();
    let mut rows = vec![];
    for (y, line) in line_iterator.enumerate() {
        if line.is_empty() {
            reading_grid = false;
        } else if reading_grid {
            let mut row: Vec<char> = vec![];
            for (x, char) in line.chars().enumerate() {
                if char == '@' {
                    robot_pos.x = x.try_into().unwrap();
                    robot_pos.y = y.try_into().unwrap();
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

    let mut grid = Grid::new(rows);

    println!("Robot starts at {}, {}", robot_pos.x, robot_pos.y);
    for command in directions.chars() {
        let (dx, dy) = match command {
            '>' => (1,0),
            '<' => (-1,0),
            '^' => (0,-1),
            'v' => (0,1),
            _ => { panic!("Unknown direction command {command}");}
        };
        if test_move(&mut grid, &robot_pos, dx, dy) {
            assert!(actually_move(&mut grid, &robot_pos, dx, dy));
            println!("{}: success", command);
            robot_pos.x += dx;
            robot_pos.y += dy;
        } else {
            println!("{}: failed", command);
        }
        grid.dump();
    }
    println!("Final GPS sum: {}", gps(&grid));
    Ok(())
}
