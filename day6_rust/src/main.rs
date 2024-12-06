use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum GridElement {
    Blocked,
    Free,
}

struct Grid {
    rows: Vec<Vec<GridElement>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<GridElement> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(self.rows[TryInto::<usize>::try_into(y).unwrap()][TryInto::<usize>::try_into(x).unwrap()])
        }
    }
    fn set(&mut self, x: i32, y: i32, newsymbol: GridElement) {
        if y < 0 || y >= self.height() || x < 0 || x>= self.width() {
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
}

enum TourResult {
    Exited { visited: usize },
    InfiniteLoop,
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
            Some(GridElement::Free) => {
                xpos = target_x;
                ypos = target_y;
            },
            Some(GridElement::Blocked) => {
                direction = (direction+3)%4
            },
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
    let mut grid_vector: Vec<Vec<GridElement>> = vec![];
    let mut start_xpos = 0i32;
    let mut ypos = 0i32;
    let mut start_ypos = 0i32;
    for row in row_iterator {

        if !row.is_empty() {
            let mut row_vector: Vec<GridElement> = vec![];
            let mut xpos = 0;
            for c in row.chars() {
                row_vector.push(match c {
                    '^' => {
                        start_xpos = xpos.try_into().unwrap();
                        start_ypos = ypos;    
                        GridElement::Free
                    },
                    '.' => GridElement::Free,
                    '#' => GridElement::Blocked,
                    x => {
                        panic!("Unknown source character {} at row {}, column {}", x, xpos, ypos);
                    }
                });
                xpos += 1;
            }
            grid_vector.push(row_vector);
            // Check all rows are the same length
            assert!(grid_vector[0].len() == row.len());
            ypos += 1;
        }
    }
    let mut grid = Grid { rows: grid_vector };
    println!("Starting at {},{}", start_xpos, start_ypos);
    let direction = 1;
    match evaluate_grid(&grid, start_xpos, start_ypos, direction) {
        TourResult::Exited{visited: x} => {
            println!("Part 1: Visited {} squares", x);
        },
        TourResult::InfiniteLoop => {
            println!("Found infinite loop");
        },
    }
    // Part 2
    let mut infinite_loop_positions = 0;
    for x in 0i32..grid.width() {
        for y in 0i32..grid.height() {
            if grid.get(x, y) == Some(GridElement::Free) {
                grid.set(x, y, GridElement::Blocked);
                match evaluate_grid(&grid, start_xpos, start_ypos, direction) {
                    TourResult::Exited { visited: _ }=> {
                    },
                    TourResult::InfiniteLoop => {
                        infinite_loop_positions += 1;
                    },
                }
                grid.set(x, y, GridElement::Free);
            }
        }
    }
    println!("Part 2: Found {} positions which give infinite loops", infinite_loop_positions);
    Ok(())
}
