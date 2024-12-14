use std::fs::File;
use std::io::prelude::*;

fn parse_field(text: &str) -> i128 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StoneCount {
    number: i128,
    count: i64,
}

impl PartialOrd for StoneCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

impl Ord for StoneCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

fn blink(stones: &Vec<StoneCount>) -> Vec<StoneCount> {
    let mut newcount = Vec::<StoneCount>::new();
    for s in stones {
        let num_as_string = s.number.to_string();
        if s.number == 0 {
            newcount.push(StoneCount {
                number: 1,
                count: s.count,
            });
        } else if num_as_string.len() % 2 == 0 {
            let splitpoint = num_as_string.len() / 2;
            newcount.push(StoneCount {
                number: parse_field(&num_as_string[..splitpoint]),
                count: s.count,
            });
            newcount.push(StoneCount {
                number: parse_field(&num_as_string[splitpoint..]),
                count: s.count,
            });
        } else {
            newcount.push(StoneCount {
                number: s.number * 2024,
                count: s.count,
            });
        }
    }
    newcount
}

fn collapse(stones: &mut Vec<StoneCount>) -> Vec<StoneCount> {
    let mut newcount = Vec::<StoneCount>::new();
    if stones.is_empty() {
        panic!("Stones vector is empty!");
    }
    newcount.push(stones.pop().unwrap());
    while let Some(s) = stones.pop() {
        if let Some(comparison_stone) = newcount.last_mut() {
            if comparison_stone.number == s.number {
                comparison_stone.count += s.count;
            } else {
                newcount.push(s);
            }
        }
    }
    newcount
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input11.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let fields = contents.strip_suffix("\n").unwrap().split(" ");

    let mut stones = Vec::<StoneCount>::new();
    for f in fields {
        stones.push(StoneCount {
            number: parse_field(f),
            count: 1,
        });
    }

    for _i in 0..75 {
        let mut newstones1 = blink(&stones);
        newstones1.sort();
        let newstones2 = collapse(&mut newstones1);

        stones = newstones2;
        println!("{:?}", stones);
    }

    let mut stonecount = 0;
    for s in stones {
        stonecount += s.count;
    }
    println!("Stone count: {}", stonecount);
    Ok(())
}
