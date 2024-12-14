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
}

pub fn parse_field(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}
