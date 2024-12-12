use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::env;

type Point = (i32, i32);

struct Grid {
    rows: Vec<Vec<char>>,
}

struct FenceCollection {
    top: HashSet::<Point>,
    bottom: HashSet::<Point>,
    left: HashSet::<Point>,
    right: HashSet::<Point>
}

impl FenceCollection {
    fn new() -> FenceCollection {
        FenceCollection {
            top: HashSet::<Point>::new(),
            bottom: HashSet::<Point>::new(),
            left: HashSet::<Point>::new(),
            right: HashSet::<Point>::new(),
        }
    }
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
    #[allow(dead_code)]
    fn inside(&self, position: Point) -> bool {
        let (x, y) = position;
        x >=0 && x < self.width() && y >= 0 && y < self.height()
    }
}

fn scan(grid: &Grid, x: i32, y: i32, visited: &mut HashSet<Point>,
        fences: &mut FenceCollection) -> Point{
    if visited.get(&(x, y)).is_some() {
        
        (0,0)
    } else if let Some(basechar) = grid.get(x, y) {
        let mut perimeter = 0;
        let mut area = 0;
        visited.insert ((x,y));
        let directions = [ (1,0), (0,-1), (-1, 0), (0, 1)];
        for (dx, dy) in directions {
            match grid.get(x+dx, y+dy) {
                None => { 
                    match (dx, dy) {
                        (1,0) => { fences.right.insert((x+dx, y+dy)); },
                        (-1,0) => { fences.left.insert((x+dx, y+dy)); },
                        (0,-1) => { fences.top.insert((x+dx, y+dy)); },
                        (0,1) => { fences.bottom.insert((x+dx, y+dy)); },
                        _ => { panic!();}
                    };
                    perimeter += 1; },
                Some(c) => {
                    if c == basechar {
                        let (a, p) = scan(grid, x+dx, y+dy, visited, fences);
                        perimeter += p;
                        area += a;                    
                    } else {
                        match (dx, dy) {
                            (1,0) => { fences.right.insert((x+dx, y+dy)); },
                            (-1,0) => { fences.left.insert((x+dx, y+dy)); },
                            (0,-1) => { fences.top.insert((x+dx, y+dy)); },
                            (0,1) => { fences.bottom.insert((x+dx, y+dy)); },
                            _ => { panic!();}
                        };
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

fn clear(grid: &mut Grid, x: i32, y:i32) {
    if let Some(basechar) = grid.get(x, y) {
        grid.set(x, y, '.');
        let directions = [ (1,0), (0,-1), (-1, 0), (0, 1)];
        for (dx, dy) in directions {
            if let Some(c) = grid.get(x+dx, y+dy) {
                if c == basechar {
                    clear(grid, x+dx, y+dy);
                }
            }
        }
    } else {
        panic!("Invalid character at {x}, {y}");
    }
}

fn count_horizontal_fences(fences: &HashSet::<Point>, width: i32, height: i32) -> i32 {
    let mut on_fence = false;
    let mut fence_count = 0;
    for y in -1..height+1 {
        for x in -1..width+1 {
            if fences.get(&(x,y)).is_some() {
                if !on_fence {
                    fence_count += 1;
                    on_fence = true;
                }
            } else {
                on_fence = false;
            }
        }
    }
    fence_count
}

fn count_vertical_fences(fences: &HashSet::<Point>, width: i32, height: i32) -> i32 {
    let mut on_fence = false;
    let mut fence_count = 0;
    for x in -1..width+1 {
        for y in -1..height+1 {
                if fences.get(&(x,y)).is_some() {
                if !on_fence {
                    fence_count += 1;
                    on_fence = true;
                }
            } else {
                on_fence = false;
            }
        }
    }
    fence_count
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
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
    let mut total_score2 = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let mut fences: FenceCollection = FenceCollection::new();
            if grid.get(x,y) != Some('.') {
                let (area,perimeter) = scan(&grid, x,y, &mut HashSet::<Point>::new(), &mut fences);
                total_score += area*perimeter;
                clear(&mut grid, x, y);
                let mut fence_length = 0;
                for f in [fences.top, fences.bottom] {
                    fence_length += count_horizontal_fences (&f, grid.width(), grid.height());
                }
                for f in [fences.left, fences.right] {
                    fence_length += count_vertical_fences (&f, grid.width(), grid.height());
                }
                println!("Scan {x},{y} -> {area}, {perimeter} fence_length = {}", fence_length);
                total_score2 += area*fence_length;
            }
        }

    }
    println!("Total score {total_score}");
    println!("Total score (part 2) {total_score2}");
    Ok(())
}
