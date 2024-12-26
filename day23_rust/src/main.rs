use arraystring::{typenum::U2, ArrayString};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
mod aoc_utils;

type ComputerName = ArrayString<U2>;

fn find_connections(
    pairs: &Vec<(ComputerName, ComputerName)>,
    first_machine: ComputerName,
) -> HashSet<ComputerName> {
    let mut connections = HashSet::<ComputerName>::new();
    for (m1, m2) in pairs {
        if *m1 == first_machine {
            connections.insert(*m2);
        } else if *m2 == first_machine {
            connections.insert(*m1);
        }
    }
    connections
}

fn connected(
    pairs: &Vec<(ComputerName, ComputerName)>,
    first_machine: ComputerName,
    second_machine: ComputerName,
) -> bool {
    for (m1, m2) in pairs {
        if *m1 == first_machine && *m2 == second_machine {
            return true;
        }
    }
    false
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let line_iterator = contents.split('\n');
    let mut pairs = Vec::<(ComputerName, ComputerName)>::new();
    let mut computers = HashSet::<ComputerName>::new();
    for line in line_iterator {
        if !line.is_empty() {
            let mut field_iterator = line.split('-');
            let machine1: ComputerName = field_iterator.next().unwrap().into();
            let machine2: ComputerName = field_iterator.next().unwrap().into();
            pairs.push((machine1, machine2));
            computers.insert(machine1);
            computers.insert(machine2);
        }
    }

    let mut triples = HashSet::<Vec<ComputerName>>::new();
    let mut highest_edges = 0;
    let mut most_indirect_connections = 0;
    for computer in computers {
        let connections: Vec<ComputerName> =
            find_connections(&pairs, computer).into_iter().collect();
        if connections.len() > highest_edges {
            highest_edges = connections.len();
        }
        let mut indirect_connections = 0;
        for i in 0..connections.len() {
            for j in 1..connections.len() {
                if connected(&pairs, connections[i], connections[j]) {
                    if computer.starts_with('t')
                        || connections[i].starts_with('t')
                        || connections[j].starts_with('t')
                    {
                        let mut triple = vec![computer, connections[i], connections[j]];
                        triple.sort();
                        triples.insert(triple);
                    }
                    indirect_connections += 1;
                }
            }
        }
        println!(
            "Connections to {}: {} ({}, {} internal connections)",
            computer,
            connections.join(", "),
            connections.len(),
            indirect_connections
        );
        if indirect_connections > most_indirect_connections {
            most_indirect_connections = indirect_connections;
        }
    }
    println!(
        "There are {} groups of three connected machines.",
        triples.len()
    );
    println!("Nodes have at most {} connections.", highest_edges);
    println!(
        "Most indirection connections is {}.",
        most_indirect_connections
    );
    Ok(())
}
