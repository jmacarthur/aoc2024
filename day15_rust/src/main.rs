use std::env;
use std::fs::File;
use std::io::prelude::*;

mod aoc_utils;
use crate::aoc_utils::*;

struct Vector {
    x: i32,
    y: i32,
}

fn move_boxes(grid: &mut Grid, pos: &Vector, dx: i32, dy: i32, test_move: bool) -> bool {
    let (target_x, target_y) = (pos.x + dx, pos.y + dy);
    match grid.get(target_x, target_y) {
        Some('.') => {
            if !test_move {
                grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                grid.set(pos.x, pos.y, '.');
            }
            true
        }
        Some('O') => {
            if move_boxes(
                grid,
                &Vector {
                    x: target_x,
                    y: target_y,
                },
                dx,
                dy,
                test_move,
            ) {
                if !test_move {
                    grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                    grid.set(pos.x, pos.y, '.');
                }
                true
            } else {
                false
            }
        }
        Some('[') => {
            if dy != 0 {
                if move_boxes(
                    grid,
                    &Vector {
                        x: target_x,
                        y: target_y,
                    },
                    dx,
                    dy,
                    test_move,
                ) && move_boxes(
                    grid,
                    &Vector {
                        x: target_x + 1,
                        y: target_y,
                    },
                    dx,
                    dy,
                    test_move,
                ) {
                    if !test_move {
                        grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                        grid.set(pos.x, pos.y, '.');
                    }
                    true
                } else {
                    false
                }
            } else if move_boxes(
                grid,
                &Vector {
                    x: target_x,
                    y: target_y,
                },
                dx,
                dy,
                test_move,
            ) {
                if !test_move {
                    grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                    grid.set(pos.x, pos.y, '.');
                }
                true
            } else {
                false
            }
        }
        Some(']') => {
            if dy != 0 {
                if move_boxes(
                    grid,
                    &Vector {
                        x: target_x,
                        y: target_y,
                    },
                    dx,
                    dy,
                    test_move,
                ) && move_boxes(
                    grid,
                    &Vector {
                        x: target_x - 1,
                        y: target_y,
                    },
                    dx,
                    dy,
                    test_move,
                ) {
                    if !test_move {
                        grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                        grid.set(pos.x, pos.y, '.');
                    }
                    true
                } else {
                    false
                }
            } else if move_boxes(
                grid,
                &Vector {
                    x: target_x,
                    y: target_y,
                },
                dx,
                dy,
                test_move,
            ) {
                if !test_move {
                    grid.set(target_x, target_y, grid.get(pos.x, pos.y).unwrap());
                    grid.set(pos.x, pos.y, '.');
                }
                true
            } else {
                false
            }
        }

        _ => false,
    }
}

fn gps(grid: &Grid) -> i32 {
    let mut total = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y) == Some('O') || grid.get(x, y) == Some('[') {
                total += 100 * y + x;
            }
        }
    }
    total
}

fn widen(grid: &Grid) -> Grid {
    let mut rows = vec![];
    for y in 0..grid.height() {
        let mut row: Vec<char> = vec![];
        for x in 0..grid.width() {
            match grid.get(x, y) {
                Some('.') => {
                    row.push('.');
                    row.push('.');
                }
                Some('#') => {
                    row.push('#');
                    row.push('#');
                }
                Some('O') => {
                    row.push('[');
                    row.push(']');
                }
                _ => {
                    panic!("Unknown character in grid");
                }
            }
        }
        rows.push(row);
    }
    Grid::new(rows)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');

    let mut robot_pos = Vector { x: 0, y: 0 };

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

    // Comment out these lines for part 1.
    grid = widen(&grid);
    robot_pos.x *= 2;

    println!("Robot starts at {}, {}", robot_pos.x, robot_pos.y);
    for command in directions.chars() {
        let (dx, dy) = match command {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => {
                panic!("Unknown direction command {command}");
            }
        };
        if move_boxes(&mut grid, &robot_pos, dx, dy, true) {
            assert!(move_boxes(&mut grid, &robot_pos, dx, dy, false));
            robot_pos.x += dx;
            robot_pos.y += dy;
            println!(
                "{}: success, now at {}, {}",
                command, robot_pos.x, robot_pos.y
            );
        } else {
            println!("{}: failed", command);
        }
        grid.dump();
    }
    println!("Final GPS sum: {}", gps(&grid));
    Ok(())
}
