use std::fs::File;
use std::io::prelude::*;

struct Grid<'a> {
    rows: Vec<&'a str>,
}

impl<'a> Grid<'a> {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height() || x < 0 {
            // No need to check x >= width, as nth will just return None
            None
        } else {
            self.rows[TryInto::<usize>::try_into(y).unwrap()]
                .chars()
                .nth(x.try_into().unwrap())
        }
    }
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
}

fn find_words(grid: &Grid, startx: i32, starty: i32, target: &str) -> u32 {
    let wordsearch_directions = [
        (1i32, 0i32),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut match_count = 0;
    for (dx, dy) in wordsearch_directions {
        let found_word = 'search: {
            for step in 0i32..4 {
                let step_usize = TryInto::<usize>::try_into(step).unwrap();
                if grid.get(startx + dx * step, starty + dy * step)
                    != target.chars().nth(step_usize)
                {
                    break 'search false;
                }
            }
            true
        };
        if found_word {
            println!("Match found at {}, {}", startx, starty);
            match_count += 1;
        }
    }
    match_count
}

fn find_crosses(grid: &Grid, startx: i32, starty: i32) -> u32 {
    let cross_directions = [(1i32, 1i32), (-1, 1)];
    let mut match_count = 0;
    if grid.get(startx, starty) == Some('A') {
        let found_cross = 'search: {
            for (dx, dy) in cross_directions {
                let c1 = grid.get(startx + dx, starty + dy);
                let c2 = grid.get(startx - dx, starty - dy);
                if !((c1 == Some('S') && c2 == Some('M')) || (c1 == Some('M') && c2 == Some('S'))) {
                    break 'search false;
                }
            }
            true
        };
        if found_cross {
            println!("Cross match found at {}, {}", startx, starty);
            match_count += 1;
        }
    }
    match_count
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut row_vector = vec![];
    for row in row_iterator {
        if !row.is_empty() {
            row_vector.push(row);

            // Check all rows are the same length
            assert!(row_vector[0].len() == row.len());
        }
    }
    let grid = Grid { rows: row_vector };

    let target = "XMAS";
    let mut match_count = 0;
    for startx in 0i32..grid.width() {
        for starty in 0i32..grid.height() {
            match_count += find_words(&grid, startx, starty, target);
        }
    }

    let mut cross_match_count = 0;
    for startx in 0i32..grid.width() {
        for starty in 0i32..grid.height() {
            cross_match_count += find_crosses(&grid, startx, starty);
        }
    }

    println!("Total wordsearch matches: {}", match_count);
    println!("Total crossmas matches: {}", cross_match_count);
    Ok(())
}
