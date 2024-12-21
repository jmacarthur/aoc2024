type Point = (i32, i32);

pub struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(rows: Vec<Vec<char>>) -> Grid {
        Grid { rows }
    }
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            None
        } else {
            Some(
                self.rows[TryInto::<usize>::try_into(y).unwrap()]
                    [TryInto::<usize>::try_into(x).unwrap()],
            )
        }
    }
    pub fn set(&mut self, x: i32, y: i32, newsymbol: char) {
        if y < 0 || y >= self.height() || x < 0 || x >= self.width() {
            return;
        }
        let row = TryInto::<usize>::try_into(y).unwrap();
        let col = TryInto::<usize>::try_into(x).unwrap();
        self.rows[row][col] = newsymbol;
    }
    pub fn width(&self) -> i32 {
        self.rows[0].len().try_into().unwrap()
    }
    pub fn height(&self) -> i32 {
        self.rows.len().try_into().unwrap()
    }
    #[allow(dead_code)]
    pub fn inside(&self, position: Point) -> bool {
        let (x, y) = position;
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }

    pub fn dump(&self) {
        for l in &self.rows {
            let mut row = String::new();
            for c in l {
                row.push(*c);
            }
            println!("{}", row);
        }
    }
}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        let newrows = self.rows.clone();
        Grid::new(newrows)
    }
}

pub fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> Grid {
        Grid::new(vec![
            "#..#".chars().collect(),
            "#..#".chars().collect(),
            "#..#".chars().collect(),
            "#..#".chars().collect(),
        ])
    }

    #[test]
    fn small_grids() {
        let grid = Grid::new(vec![]);
        assert_eq!(grid.height(), 0);

        let grid2 = Grid::new(vec![vec![]]);
        assert_eq!(grid2.height(), 1);
        assert_eq!(grid2.width(), 0);

        let grid3 = Grid::new(vec![vec!['#', '.', '.', '.']]);
        assert_eq!(grid3.height(), 1);
        assert_eq!(grid3.width(), 4);
    }

    #[test]
    fn out_of_bounds() {
        let grid = test_grid();
        assert_eq!(grid.height(), 4);
        assert_eq!(grid.width(), 4);

        assert_eq!(grid.get(-1, -1), None);
        assert_eq!(grid.get(0, -1), None);
        assert_eq!(grid.get(-1, 0), None);
        assert_eq!(grid.get(4, 1), None);
        assert_eq!(grid.get(1, 4), None);
        assert_eq!(grid.get(5, 5), None);
    }
    #[test]
    fn get() {
        let grid = test_grid();
        assert_eq!(grid.get(0, 0), Some('#'));
        assert_eq!(grid.get(1, 0), Some('.'));
        assert_eq!(grid.get(0, 1), Some('#'));
    }
    #[test]
    fn set() {
        let mut grid = test_grid();
        assert_eq!(grid.get(1, 2), Some('.'));
        grid.set(1, 2, '#');
        assert_eq!(grid.get(1, 2), Some('#'));
        assert_eq!(grid.get(2, 1), Some('.'));
    }
    #[test]
    fn clone() {
        let mut grid = test_grid();
        assert_eq!(grid.get(1, 2), Some('.'));
        let grid2 = grid.clone();
        grid.set(1, 2, '#');
        assert_eq!(grid.get(1, 2), Some('#'));
        assert_eq!(grid2.get(1, 2), Some('.'));
    }
}
