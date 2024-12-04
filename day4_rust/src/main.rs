use std::fs::File;
use std::io::prelude::*;

fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

struct Grid<'a> {
    rows: Vec<&'a str>
}

impl <'a> Grid<'a> {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || TryInto::<usize>::try_into(y).unwrap() >= self.rows.len() || x < 0 {
            None
        } else {
            self.rows[TryInto::<usize>::try_into(y).unwrap()].chars().nth(x.try_into().unwrap())
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut row_vector = vec![];
    for row in row_iterator {
        if row.len() > 0 {
            row_vector.push(row);
            assert!(row_vector[0].len() == row.len());
        }
    }
    let mut grid = Grid { rows: row_vector };
    
    let wordsearch_directions = [ (1i32,0i32), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1) ];
    let target = "XMAS";
    let mut match_count = 0;
    for startx in 0..grid.rows[0].len() {
        let startx_i: i32 = startx.try_into().unwrap();        
        for starty in 0..grid.rows.len() {
            let starty_i: i32 = starty.try_into().unwrap();
            for (dx, dy) in wordsearch_directions {
                if 'search: loop {
                    for step in 0i32..4 {
                        let step_usize = TryInto::<usize>::try_into(step).unwrap();
                        if grid.get(startx_i + dx * step, starty_i + dy * step) != target.chars().nth(step_usize) {
                            break 'search false;
                        }
                    }
                    break 'search true;
                } {
                    println!("Match found at {}, {}", startx, starty);
                    match_count += 1;
                }
            }
        }
    }

    let cross_directions = [ (1i32, 1i32), (-1,1) ];
    let mut cross_match_count = 0;
    for startx in 0..grid.rows[0].len() {
        let startx_i: i32 = startx.try_into().unwrap();        
        for starty in 0..grid.rows.len() {
            let starty_i: i32 = starty.try_into().unwrap();
            if grid.get(startx_i, starty_i) == Some('A') {
                let mut matches = true;
                for (dx, dy) in cross_directions {
                    if grid.get(startx_i + dx, starty_i + dy) == Some('M') && grid.get(startx_i - dx, starty_i - dy) == Some('S') {
                    } else if grid.get(startx_i + dx, starty_i + dy) == Some('S') && grid.get(startx_i - dx, starty_i - dy) == Some('M') {
                    } else {
                        matches = false;
                    }
                }
                if matches {
                    println!("Cross match found at {}, {}", startx, starty);
                    cross_match_count += 1;
                }
            }
        }
    }

    println!("Total wordsearch matches: {}", match_count);
    println!("Total crossmas matches: {}", cross_match_count);
    Ok(())
}
