use std::fs::File;
use std::io::prelude::*;

fn parse_int(text: &str) -> i64 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

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

fn test_equation(equation: &Equation, num_operators: i64) -> bool {
    let combinations =
        num_operators.pow(TryInto::<u32>::try_into(equation.operands.len() - 1).unwrap());
    for c in 0..combinations {
        let mut operand_iterator = equation.operands.iter();
        let mut total = *operand_iterator.next().unwrap();
        for (step, operand) in operand_iterator.enumerate() {
            // Select the operator for this step. It would be a lot simpler and arguably clearer just to
            // match on an integer here, but this is an exercise for me to make more use of enumerations.
            let op: Operator = ((c / num_operators.pow(TryInto::<u32>::try_into(step).unwrap()))
                % num_operators)
                .try_into()
                .unwrap();
            total = match op {
                Operator::Add => total + operand,
                Operator::Multiply => total * operand,
                Operator::Concatenate => numeric_concat(total, operand),
            };

            // Since all operands are 1 or above, no operation can make the total smaller, so stop processing here. We could knock out a lot more combinations
            // this way, but the logic becomes much more complicated.
            if total > equation.target {
                break;
            };
        }

        if total == equation.target {
            return true;
        }
    }
    false
}

fn search_equations(equation_vector: &Vec<Equation>, num_operators: i64) -> i64 {
    let mut valid_target_total = 0;
    for equation in equation_vector {
        if test_equation(equation, num_operators) {
            valid_target_total += equation.target;
        }
    }
    valid_target_total
}

fn read_file_to_lines() -> std::io::Result<String> {
    let mut file = File::open("input7.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let mut equation_vector: Vec<Equation> = vec![];
    let file_contents = read_file_to_lines().unwrap();
    for line in file_contents.split("\n") {
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
                let val = parse_int(operand);

                // Check all values are 1 or above. This is observed in the input and useful for an optimisation later.
                assert!(val > 0);

                operand_vector.push(val);
            }
            equation_vector.push(Equation {
                target,
                operands: operand_vector,
            });
        }
    }

    println!(
        "Sum of all valid targets with 2 operators: {}",
        search_equations(&equation_vector, 2)
    );
    println!(
        "Sum of all valid targets with 3 operators: {}",
        search_equations(&equation_vector, 3)
    );
    Ok(())
}

#[test]
fn test_basic() {
    assert_eq!(
        test_equation(
            &Equation {
                target: 4,
                operands: vec![2, 2]
            },
            2
        ),
        true
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 100,
                operands: vec![2, 2]
            },
            2
        ),
        false
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 100,
                operands: vec![2, 2, 25]
            },
            2
        ),
        true
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 4,
                operands: vec![2, 2]
            },
            3
        ),
        true
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 22,
                operands: vec![2, 2]
            },
            2
        ),
        false
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 22,
                operands: vec![2, 2]
            },
            3
        ),
        true
    );
}

#[test]
fn test_advanced() {
    assert_eq!(
        test_equation(
            &Equation {
                target: 12,
                operands: vec![2, 2, 3]
            },
            2
        ),
        true
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 610,
                operands: vec![2, 3, 10]
            },
            3
        ),
        true
    );
    assert_eq!(
        test_equation(
            &Equation {
                target: 410,
                operands: vec![2, 3, 10]
            },
            3
        ),
        false
    );
}
