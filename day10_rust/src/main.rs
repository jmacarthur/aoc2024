use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn char_to_int(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        x => {
            panic!("Unreadable character {}", x)
        }
    }
}

struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(
                self.rows[TryInto::<usize>::try_into(y).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()],
            )
        }
    }
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
}

fn trace_route(grid: &Grid, x: i32, y: i32, heads: &mut Vec<(i32, i32)>) {
    let start_height = match grid.get(x, y) {
        Some(h) => h,
        None => {
            panic!("Unreadable grid position ({},{})", x, y);
        }
    };
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    for (dx, dy) in directions {
        if let Some(h2) = grid.get(x + dx, y + dy) {
            if h2 == start_height + 1 {
                if h2 == 9 {
                    println!("Found top at ({}, {})", x + dx, y + dy);
                    heads.push((x + dx, y + dy));
                } else {
                    trace_route(grid, x + dx, y + dy, heads);
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input10.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut grid_vector: Vec<Vec<u8>> = vec![];
    for row in row_iterator {
        if !row.is_empty() {
            let mut row_vector: Vec<u8> = vec![];
            for c in row.chars() {
                row_vector.push(char_to_int(c));
            }
            grid_vector.push(row_vector);
            // Check all rows are the same length
            assert!(grid_vector[0].len() == row.len());
        }
    }

    let grid = Grid { rows: grid_vector };

    let mut part1_score = 0;
    let mut part2_score = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let mut heads = Vec::<(i32, i32)>::new();
            if grid.get(x, y) == Some(0) {
                trace_route(&grid, x, y, &mut heads);
                println!("Trailhead at ({}, {}) to ({:?})", x, y, heads);
                part2_score += heads.len();
                let mut unique_heads = HashSet::<(i32, i32)>::new();
                for head in heads {
                    unique_heads.insert(head);
                }
                part1_score += unique_heads.len();
            }
        }
    }
    println!("Part 1 score: {}", part1_score);
    println!("Part 2 score: {}", part2_score);

    Ok(())
}
