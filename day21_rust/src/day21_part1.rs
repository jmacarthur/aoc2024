use crate::aoc_utils::*;
static PART1_STEPS: i32 = 2;

pub fn numeric_pad_positions(c: char) -> (i32, i32) {
    // Maps the numeric characters 0-9 and A to x,y coordinates on the
    // numeric keypad grid.
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => {
            panic!("No character {c} on the numeric pad");
        }
    }
}

pub fn directional_pad_positions(c: char) -> (i32, i32) {
    // Maps the directional characters ^v<> and A to x,y coordinates on the
    // directional keypad grid.
    match c {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => {
            panic!("No character {c} on the directional pad");
        }
    }
}

fn reverse_numeric(x: i32, y: i32) -> Option<char> {
    // Reverses numeric_pad_positions; finds the character at coordinates x,y on
    // the numeric keypad.
    "0123456789A"
        .chars()
        .find(|&c| numeric_pad_positions(c) == (x, y))
}

fn reverse_directional(x: i32, y: i32) -> Option<char> {
    // Reverses directional_pad_positions; finds the character at coordinates x,y on
    // the directional keypad.
    "^<>vA"
        .chars()
        .find(|&c| directional_pad_positions(c) == (x, y))
}

pub fn find_route(from: char, to: char, position: impl Fn(char) -> (i32, i32)) -> Vec<char> {
    // Find the best route from one character to another on a supplied grid.
    // position should be directional_pad_positions or numeric_pad_positions.

    // 'Best' is a choice between horizontal first and vertical first. We never interleave
    // horizontal and vertical (e.g. "<v<") because that will always be more work for the next
    // keypad operator. Sometimes we have to go horizontal first or vertical first to avoid
    // the empty square on a grid. Otherwise, we prefer to do horizontal first if moving left,
    // which reduces trips to the furthest away button.
    let mut route = vec![];
    let (from_x, from_y) = position(from);
    let (to_x, to_y) = position(to);
    let dx = (to_x - from_x).signum();
    let dy = (to_y - from_y).signum();
    let mut x = from_x;
    let mut y = from_y;
    let mut vertical_moves = vec![];
    let mut horizontal_moves = vec![];
    while x != to_x {
        if dx < 0 {
            horizontal_moves.push('<');
            x -= 1;
        } else {
            horizontal_moves.push('>');
            x += 1;
        }
    }

    while y != to_y {
        if dy < 0 {
            vertical_moves.push('^');
            y -= 1;
        } else {
            vertical_moves.push('v');
            y += 1;
        }
    }

    if to == '<' || (from_y == 3 && to_x == 0) {
        // Must go vertical first
        route.extend(vertical_moves);
        route.extend(horizontal_moves);
    } else if from == '<' || (from_x == 0 && to_y == 3) {
        // Must go horizontal first
        route.extend(horizontal_moves);
        route.extend(vertical_moves);
    } else {
        // Prefer '<' first
        if dx < 0 {
            route.extend(horizontal_moves);
            route.extend(vertical_moves);
        } else {
            route.extend(vertical_moves);
            route.extend(horizontal_moves);
        }
    }
    route.push('A');
    route
}

fn check_route_directional(route: &Vec<char>) -> Vec<char> {
    // Return the result of tapping 'route' on a directional keypad.
    let (mut x, mut y) = directional_pad_positions('A');
    let mut result = vec![];
    for c in route {
        match c {
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            '^' => {
                y -= 1;
            }
            'v' => {
                y += 1;
            }
            'A' => {
                result.push(reverse_directional(x, y).unwrap());
            }
            _ => {
                panic!("Invalid char in sequence {c}");
            }
        }
        if reverse_directional(x, y).is_none() {
            panic!("Robot hit invalid directional position {x}, {y}");
        }
    }
    result
}

fn check_route_numeric(route: &Vec<char>) -> Vec<char> {
    // Return the result of tapping 'route' on a numeric keypad.
    let (mut x, mut y) = numeric_pad_positions('A');
    let mut result = vec![];
    for c in route {
        match c {
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            '^' => {
                y -= 1;
            }
            'v' => {
                y += 1;
            }
            'A' => {
                result.push(reverse_numeric(x, y).unwrap());
            }
            _ => {
                panic!("Invalid char in sequence {c}");
            }
        }
        if reverse_numeric(x, y).is_none() {
            panic!("Robot hit invalid numeric position {x}, {y}");
        }
    }
    result
}

fn find_sequence(sequence: &Vec<char>) -> Vec<char> {
    let mut route1 = vec![]; // Route on first directional pad
    route1.extend(find_route('A', sequence[0], numeric_pad_positions));
    for pair in sequence.windows(2) {
        route1.extend(find_route(pair[0], pair[1], numeric_pad_positions));
    }
    let mut previous_route = route1;
    let mut route: Vec<char> = vec![];
    for _i in 0..PART1_STEPS {
        route = vec![]; // Route on second directional pad
        route.extend(find_route(
            'A',
            previous_route[0],
            directional_pad_positions,
        ));
        for pair in previous_route.windows(2) {
            route.extend(find_route(pair[0], pair[1], directional_pad_positions));
        }
        previous_route = route.clone();
    }
    route
}

pub fn solve_part1(sequences: &Vec<&str>) -> usize {
    let mut total_checksum = 0;
    for s in sequences {
        let num: usize = parse_field(&s[0..3]).try_into().unwrap();
        let route = find_sequence(&s.chars().collect());
        total_checksum += route.len() * num;
        println!(
            "{:?} -> {:?}, len {}, checksum {}",
            s,
            route.iter().cloned().collect::<String>(),
            route.len(),
            route.len() * num
        );
        let mut validate = check_route_directional(&route);

        for _i in 1..PART1_STEPS {
            validate = check_route_directional(&validate);
        }
        validate = check_route_numeric(&validate);
        println!(
            "Validate result of final sequence: {:?}",
            validate.iter().cloned().collect::<String>()
        );
    }
    println!("Total checksum {}", total_checksum);
    total_checksum
}
