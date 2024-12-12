use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(
                self.rows[TryInto::<usize>::try_into(y).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()],
            )
        }
    }
    fn set(&mut self, x: i32, y: i32, newsymbol: char) {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            return;
        }
        let row = TryInto::<usize>::try_into(y).unwrap();
        let col = TryInto::<usize>::try_into(x).unwrap();
        self.rows[row][col] = newsymbol;
    }
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
    fn inside(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;
        x >=0 && x < self.width() && y >= 0 && y < self.height()
    }
}

fn scan(grid: &Grid, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>) -> (i32, i32) {
    if visited.get(&(x, y)) != None {
        
        (0,0)
    } else {
        if let Some(basechar) = grid.get(x, y) {

            let mut perimeter = 0;
            let mut area = 0;
            visited.insert ((x,y));
            let directions = [ (1,0), (0,-1), (-1, 0), (0, 1)];
            for (dx, dy) in directions {
                match grid.get(x+dx, y+dy) {
                    None => { perimeter += 1; },
                    Some(c) => {
                        if c == basechar {
                            let (a, p) = scan(grid, x+dx, y+dy, visited);
                            perimeter += p;
                            area += a;                    
                        } else {
                            perimeter += 1;
                        }
                    }
                }
            }
            (area + 1, perimeter)
        } else {
            panic!("Invalid character at {x}, {y}");
        }
    }
}

fn clear(grid: &mut Grid, x: i32, y:i32) {
    if let Some(basechar) = grid.get(x, y) {
        grid.set(x, y, '.');
        let directions = [ (1,0), (0,-1), (-1, 0), (0, 1)];
        for (dx, dy) in directions {
            match grid.get(x+dx, y+dy) {
                Some(c) => {
                    if c == basechar {
                        clear(grid, x+dx, y+dy);
                    }
                },
                _ => ()
            }
        }
    } else {
        panic!("Invalid character at {x}, {y}");
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input12.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut grid_vector: Vec<Vec<char>> = vec![];
    for row in row_iterator {
        if !row.is_empty() {
            let mut row_vector: Vec<char> = vec![];
            for c in row.chars() {
                row_vector.push(c);                
            }
            grid_vector.push(row_vector);
            // Check all rows are the same length
            assert!(grid_vector[0].len() == row.len());
        }
    }
    let mut grid = Grid { rows: grid_vector };
    
    
    let mut total_score = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.get(x,y) != Some('.') {
                let (a,p) = scan(&grid, x,y, &mut HashSet::<(i32, i32)>::new());
                println!("Scan {x},{y} -> {a}, {p}");
                total_score += a*p;
                clear(&mut grid, x, y);
            }
        }

    }
    println!("Total score {total_score}");
    Ok(())
}
