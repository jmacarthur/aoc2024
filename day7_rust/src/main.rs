use std::fs::File;
use std::io::prelude::*;
use strum_macros::EnumIter;

fn parse_int(text: &str) -> i64 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

#[derive(EnumIter)]
enum Operator {
    Add = 0,
    Multiply = 1,
    Concatenate = 2,
}

impl TryFrom<i64> for Operator {
    type Error = ();

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == Operator::Add as i64 => Ok(Operator::Add),
            x if x == Operator::Multiply as i64 => Ok(Operator::Multiply),
            x if x == Operator::Concatenate as i64 => Ok(Operator::Concatenate),
            _ => Err(()),
        }
    }
}

struct Equation {
    target: i64,
    operands: Vec<i64>,
}

fn numeric_concat(a: i64, b: &i64) -> i64 {
    parse_int(&(a.to_string() + &b.to_string()))
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input7.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut equation_vector: Vec<Equation> = vec![];
    for line in line_iterator {
        if !line.is_empty() {
            let mut field_iterator = line.split(":");
            let target = match field_iterator.next() {
                Some(s) => parse_int(s),
                None => {
                    panic!("Missing target field");
                }
            };
            let mut operand_vector: Vec<i64> = vec![];
            let operand_iterator = match field_iterator.next() {
                Some(s) => s.split_whitespace(),
                None => {
                    panic!("Missing operand fields");
                }
            };
            for operand in operand_iterator {
                operand_vector.push(parse_int(operand));
            }
            equation_vector.push(Equation {
                target,
                operands: operand_vector,
            });
        }
    }

    let mut valid_target_total = 0;
    let operators: i64 = 3; // Change this to 2 for part 1.
    for equation in equation_vector {
        let combinations =
            operators.pow(TryInto::<u32>::try_into(equation.operands.len() - 1).unwrap());
        for c in 0..combinations {
            let mut operand_iterator = equation.operands.iter();
            let mut total = *operand_iterator.next().unwrap();
            for (step, operand) in operand_iterator.enumerate() {
                let op: Operator = ((c / operators.pow(TryInto::<u32>::try_into(step).unwrap()))
                    % operators)
                    .try_into()
                    .unwrap();
                total = match op {
                    Operator::Add => total + operand,
                    Operator::Multiply => total * operand,
                    Operator::Concatenate => numeric_concat(total, operand),
                }
            }
            if total == equation.target {
                println!("Total {} vs target {}", total, equation.target);
                valid_target_total += equation.target;
                break;
            }
        }
    }
    println!("Sum of all valid targets: {}", valid_target_total);
    Ok(())
}
