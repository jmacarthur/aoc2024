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
    fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
}

enum TourResult {
    Exited,
    InfiniteLoop,
    Error
}

fn evaluate_grid(grid, start_xpos, start_ypos, start_direction) -> TourResult {
    let direction_delta = [ (1i32,0i32), (0, -1), (-1, 0), (0, 1) ];
    let mut xpos = xpos;
    let mut ypos = ypos;
    let mut direction = start_direction;
    loop {
        visited.insert((xpos, ypos));
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
                panic!("Unrecognised character in grid {}", e);
            }
            None => {
                println!("Exited grid from position {}, {}, direction {}", xpos, ypos, direction);
                println!("Visited {} squares", visited.len());
                break;
            }
        }
        if xpos == start_xpos && ypos == start_ypos && direction == start_direction {
            println!("Found infinite loop");
            break;
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
    let grid = Grid { rows: row_vector };
    println!("Starting at {},{}", start_xpos, start_ypos);
    let mut xpos = start_xpos;
    ypos = start_ypos;
    let mut direction = 1;
    let mut visited = HashSet::<(i32, i32)>::new();
    evaluate_grid(grid, start_xpos, start_ypos, direction);
    Ok(())
}
