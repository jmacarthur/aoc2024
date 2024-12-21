fn numeric_pad_positions(c: char) -> (i32, i32) {
    match c {
        '7' => (0,0),
        '8' => (1,0),
        '9' => (2,0),
        '4' => (0,1),
        '5' => (1,1),
        '6' => (2,1),
        '1' => (0,2),
        '2' => (1,2),
        '3' => (2,2),
        '0' => (1,3),
        'A' => (2,3),
        _ => { panic!("No character {c} on the numeric pad"); },
    }
}

fn directional_pad_positions(c: char) -> (i32, i32) {
    match c {
        '^' => (1,0),
        'A' => (2,0),
        '<' => (0,1),
        'v' => (1,1),
        '>' => (2,1),
        _ => { panic!("No character {c} on the directional pad"); },
    }
}


fn add_pad_routes(from_x: i32, from_y: i32, to_x: i32, to_y: i32, routes: &mut Vec::<Vec::<char>>, prefix: &mut Vec<char>) {
    let dx = (to_x - from_x).signum();
    let dy = (to_y - from_y).signum();
    let to_next = match (dx, dy) {
        (-1,-1) => ['<', '^'],
        (0,-1) => ['^', '.'],
        (1,-1) => ['>', '^'],
        (-1,0) => ['<', '.'],
        (0,0) => ['.', '.'],
        (1,0) => ['>', '.'],
        (-1,1) => ['<','v'],
        (0,1) => ['v', '.'],
        (1,1) => ['>', 'v'],
        _ => { panic!("Cannot move directly by ({}, {})", dx, dy);}
    };
    match to_next {
        ['.', '.'] => { let mut r = prefix.clone();
            r.push('A');
            routes.push(r); },
        [a, '.'] => {
            prefix.push(a);
            add_pad_routes(from_x + dx, from_y + dy, to_x, to_y, routes, prefix);
            prefix.pop();
        },
        [a, b] => {
            prefix.push(a);
            add_pad_routes(from_x + dx, from_y, to_x, to_y, routes, prefix);
            prefix.pop();
            prefix.push(b);
            add_pad_routes(from_x, from_y + dy, to_x, to_y, routes, prefix);
            prefix.pop();
        }
    }
}

fn numeric_pad_routes(from: char, to: char) -> Vec::<Vec::<char>> {
    let mut routes = Vec::<Vec::<char>>::new();
    let (x1, y1) = numeric_pad_positions(from);
    let (x2, y2) = numeric_pad_positions(to);
    add_pad_routes(x1, y1, x2, y2, &mut routes, &mut vec![]);
    routes
}

fn directional_pad_routes(from: char, to: char) -> Vec::<Vec::<char>> {
    let mut routes = Vec::<Vec::<char>>::new();
    let (x1, y1) = directional_pad_positions(from);
    let (x2, y2) = directional_pad_positions(to);
    add_pad_routes(x1, y1, x2, y2, &mut routes, &mut vec![]);
    routes
}


fn main() {
    println!("{:?}", numeric_pad_routes('1', '9'));
    println!("{:?}", directional_pad_routes('A', '<'));
}
