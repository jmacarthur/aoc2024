use std::fs::File;
use std::io::prelude::*;

fn parse_int(text: &str) -> i32 {
    match text.parse() {
        Ok(i) => i,
        Err(_) => {
            panic!("Unreadable number {}", text);
        }
    }
}

fn out_of_place_indexes(
    report: &[i32],
    ordering_rule_vector: &Vec<(i32, i32)>,
) -> Option<(usize, usize)> {
    for (page, successor) in ordering_rule_vector {
        'test_rule: {
            let index1 = match report.iter().position(|&r| r == *page) {
                Some(x) => x,
                None => {
                    break 'test_rule;
                }
            };
            let index2 = match report.iter().position(|&r| r == *successor) {
                Some(x) => x,
                None => {
                    break 'test_rule;
                }
            };
            if index1 > index2 {
                return Some((index1, index2));
            }
        }
    }
    None
}

fn valid(report: &[i32], ordering_rule_vector: &Vec<(i32, i32)>) -> bool {
    out_of_place_indexes(report, ordering_rule_vector).is_none()
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input5.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut ordering_rule_vector = vec![];
    let mut report_page_vector = vec![];
    let mut rules_mode = true;
    for line in line_iterator {
        if line.is_empty() {
            if rules_mode {
                rules_mode = false;
            } else {
                break;
            }
        } else if rules_mode {
            let mut rule_iterator = line.split('|');
            let match1 = rule_iterator.next().unwrap();
            let match2 = rule_iterator.next().unwrap();
            ordering_rule_vector.push((parse_int(match1), parse_int(match2)));
        } else {
            let report_iterator = line.split(',');
            let mut report = vec![];
            for field in report_iterator {
                report.push(parse_int(field));
            }
            report_page_vector.push(report);
        }
    }
    println!("{:?}", ordering_rule_vector);
    println!("{:?}", report_page_vector);

    let mut mid_total = 0;
    let mut reordered_mid_total = 0;
    for mut report in report_page_vector {
        assert!(report.len() % 2 == 1);
        let report_valid = valid(&report, &ordering_rule_vector);
        if report_valid {
            let mid = report[report.len() / 2];
            println!("{:?} => {} mid {}", report, report_valid, mid);
            mid_total += mid;
        } else {
            while let Some((l1, l2)) = out_of_place_indexes(&report, &ordering_rule_vector) {
                report.swap(l1, l2);
            }
            let mid = report[report.len() / 2];
            reordered_mid_total += mid;
        }
    }
    println!("Total of mid values for valid reports: {}", mid_total);
    println!(
        "Total of mid values for reordered reports: {}",
        reordered_mid_total
    );

    Ok(())
}
