use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

struct Grid {
    rows: Vec<String>,
}

impl Grid {
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
    fn set(&mut self, x: i32, y: i32, newchar: char) {
        if y < 0 || y >= self.height() || x < 0 || x>= self.width() {
            return;
        }
        let row = TryInto::<usize>::try_into(y).unwrap();
        let col = TryInto::<usize>::try_into(x).unwrap();
        let start = &self.rows[row][..col];
        let end = &self.rows[row][col+1..];
        let mut newrow = String::new();
        newrow.push_str(start);
        newrow.push(newchar);
        newrow.push_str(end);
        self.rows[row] = newrow;
    }
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
}

enum TourResult {
    Exited { visited: usize },
    InfiniteLoop,
    Error
}

fn evaluate_grid(grid: &Grid, start_xpos: i32, start_ypos: i32, start_direction: usize) -> TourResult {
    let direction_delta = [ (1i32,0i32), (0, -1), (-1, 0), (0, 1) ];
    let mut xpos = start_xpos;
    let mut ypos = start_ypos;
    let mut direction = start_direction;
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut visited_with_direction = HashSet::<(i32, i32, usize)>::new();
    loop {
        visited.insert((xpos, ypos));
        if visited_with_direction.contains(&(xpos, ypos, direction)) {
            return TourResult::InfiniteLoop;
        }
        visited_with_direction.insert((xpos, ypos, direction));
        let target_x = xpos + direction_delta[direction].0;
        let target_y = ypos + direction_delta[direction].1;
        match grid.get(target_x, target_y) {
            Some('.') | Some('X') => {
                xpos = target_x;
                ypos = target_y;
            },
            Some('#') => {
                direction = (direction+3)%4
            },
            Some(e) => {
                println!("Unrecognised character in grid {}", e);
                return TourResult::Error;
            }
            None => {
                //println!("Exited grid from position {}, {}, direction {}", xpos, ypos, direction);
                return TourResult::Exited{visited: visited.len()};
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut row_vector: Vec<String> = vec![];
    let mut start_xpos = 0i32;
    let mut ypos = 0i32;
    let mut start_ypos = 0i32;
    for row in row_iterator {

        if !row.is_empty() {
            match row.find('^') {
                Some(xpos) =>  {
                    let fixed_str = row.replace("^", ".");
                    row_vector.push(fixed_str);
                    start_xpos = xpos.try_into().unwrap();
                    start_ypos = ypos;
                },
                None => {
                    row_vector.push(row.to_string());
                }
            }
            // Check all rows are the same length
            assert!(row_vector[0].len() == row.len());
            ypos += 1;
        }
    }
    let mut grid = Grid { rows: row_vector };
    println!("Starting at {},{}", start_xpos, start_ypos);
    let direction = 1;
    match evaluate_grid(&grid, start_xpos, start_ypos, direction) {
        TourResult::Exited{visited: x} => {
            println!("Visited {} squares", x);
        },
        TourResult::InfiniteLoop => {
            println!("Found infinite loop");
        },
        TourResult::Error => {
            println!("Error running tour");
        }
    }
    // Part 2
    let mut infinite_loop_positions = 0;
    for x in 0i32..grid.width() {
        for y in 0i32..grid.height() {
            if grid.get(x, y) == Some('.') {
                grid.set(x, y, '#');
                match evaluate_grid(&grid, start_xpos, start_ypos, direction) {
                    TourResult::Exited { visited: _ }=> {
                    },
                    TourResult::InfiniteLoop => {
                        println!("Found infinite loop by replacing at {}, {}", x, y);
                        infinite_loop_positions += 1;
                    },
                    TourResult::Error => {
                        panic!("Error running tour");
                    }
                }
                grid.set(x, y, '.');
            }
        }
    }
    println!("That's {} new infinite loop locations", infinite_loop_positions);
    Ok(())
}
