use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum GridElement {
    Free,
    Antenna { frequency: char },
}

struct Grid {
    rows: Vec<Vec<GridElement>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<GridElement> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(
                self.rows[TryInto::<usize>::try_into(y).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()],
            )
        }
    }
    fn set(&mut self, x: i32, y: i32, newsymbol: GridElement) {
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
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("test8.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let row_iterator = contents.split('\n');
    let mut grid_vector: Vec<Vec<GridElement>> = vec![];
    for row in row_iterator {
        if !row.is_empty() {
            let mut row_vector: Vec<GridElement> = vec![];
            for c in row.chars() {
                row_vector.push(match c {
                    '.' => GridElement::Free,
                    x => GridElement::Antenna { frequency: x },
                });
            }
            grid_vector.push(row_vector);
            // Check all rows are the same length
            assert!(grid_vector[0].len() == row.len());
        }
    }
    let mut grid = Grid { rows: grid_vector };

    let mut antennas_by_frequency = HashMap::<char, Vec::<(i32, i32)>>::new();
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            match grid.get(x,y) {
                Some(GridElement::Antenna {frequency: f}) => {
                    match antennas_by_frequency.get_mut(&f) {
                        None => {
                            let location_vector = vec![(x,y)];
                            antennas_by_frequency.insert(f, location_vector);
                        },
                        Some(location_vector) => {
                            location_vector.push((x,y));
                        }                    
                    }
                }
                _ => (),
            }
        }
    }

    for f in antennas_by_frequency.keys() {
        println!("Found frequency {} = {:?}", f, antennas_by_frequency.get(f));
    }

    Ok(())
}
