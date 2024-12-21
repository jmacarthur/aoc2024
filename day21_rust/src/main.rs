use std::collections::VecDeque;

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


fn add_pad_routes(from_x: i32, from_y: i32, to_x: i32, to_y: i32,
    routes: &mut Vec::<Vec::<char>>, prefix: &mut Vec<char>
) {
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

fn combine_numeric_routes(from: char, remaining: &mut Vec::<char>) -> Vec::<Vec::<char>> {
    let mut routes = Vec::<Vec::<char>>::new();
    if remaining.is_empty() {
        return routes;
    }
    if let Some((next, remainder)) = remaining.split_first() {
        for r in numeric_pad_routes(from, *next) {
            let r2 = combine_numeric_routes(*next, &mut Vec::from(remainder));
            if r2.is_empty() {
                routes.push(r.clone());
            } else {
                for r2 in combine_numeric_routes(*next, &mut Vec::from(remainder)) {
                    routes.push(r.clone().into_iter().chain(r2.into_iter()).collect());
                }
            }
        }
    }
    routes
}

fn combine_directional_routes(from: char, remaining: &mut Vec::<char>) -> Vec::<Vec::<char>> {
    let mut routes = Vec::<Vec::<char>>::new();
    if remaining.is_empty() {
        return routes;
    }
    if let Some((next, remainder)) = remaining.split_first() {
        for r in directional_pad_routes(from, *next) {
            let r2 = combine_directional_routes(*next, &mut Vec::from(remainder));
            if r2.is_empty() {
                routes.push(r.clone());
            } else {
                for r2 in combine_directional_routes(*next, &mut Vec::from(remainder)) {
                    routes.push(r.clone().into_iter().chain(r2.into_iter()).collect());
                }
            }
        }
    }
    routes
}

fn score(route: &Vec<char>) -> i32 {
    let mut total = 0;
    let mut previous = '.';
    for c in route {
        if *c != previous {
            total += 1;
        }
        previous = *c;
    }
    total
}

fn filter(routes: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newvec = vec![];
    let mut minscore = -1;
    for route in routes {
        if minscore == -1 || score(route) < minscore {
            minscore = score(route);
        }
    }
    for route in routes {
        if score(route) == minscore {
            newvec.push(route.to_vec());
        }
    }
    newvec
}

fn main() {
    let mut sequence = Vec::from(['0', '2', '9', 'A']);
    println!("{:?}", numeric_pad_routes('1', '9'));
    println!("{:?}", directional_pad_routes('A', '<'));

    let mut l1 = combine_numeric_routes('A', &mut sequence);
    l1 = filter(&l1);
    for i in 0..2 {
        let mut l2 = vec![];
        for mut route in l1 {
            l2.extend(combine_directional_routes('A', &mut route));        
        }
        l2 = filter(&l2);
        println!("Stage {i}: len = {}", l2[0].len());
        l1 = vec![l2[0].clone()];
    }
}
