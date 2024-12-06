#![allow(dead_code)]

use std::collections::HashMap;
fn main() {
    part1::execute();
    part2::execute();
}

fn read_input() -> String {
    std::fs::read_to_string("../input/day5.txt").expect("Failed to read input at provided path.")
}

type AdjacencyList = HashMap<u8, Vec<u8>>;
type Updates = Vec<Vec<u8>>;

fn parse_input(input: &String) -> (AdjacencyList, Updates) {
    let mut p = input.split("\r\n\r\n");

    let mut adjacency_list: HashMap<u8, Vec<u8>> = std::collections::HashMap::new();

    for line in p.next().unwrap().lines() {
        let mut sp = line.split('|').filter(|v| !v.is_empty());
        let before = sp.next().unwrap().parse::<u8>().unwrap();
        let after = sp.next().unwrap().parse::<u8>().unwrap();

        adjacency_list
            .entry(before)
            .or_insert_with(Vec::new)
            .push(after);
    }

    let updates: Vec<Vec<u8>> = p
        .next()
        .into_iter()
        .flat_map(|lines| {
            lines.lines().filter_map(|line| {
                Some(
                    line.split(",")
                        .filter_map(|n| n.parse::<u8>().ok())
                        .collect(),
                )
            })
        })
        .collect();

    (adjacency_list, updates)
}

fn is_ordered(adjacency_list: &AdjacencyList, pages: &[u8]) -> bool {
    for window in pages.windows(2) {
        let (from, to) = (window[0], window[1]);

        if let Some(neighbors) = adjacency_list.get(&from) {
            if !neighbors.contains(&to) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn calc_middle_of_correct(adjacency_list: &AdjacencyList, updates: &Updates) -> u32 {
    let mut total_middle_of_correct = 0;

    for pages in updates.iter() {
        if is_ordered(&adjacency_list, pages) {
            total_middle_of_correct += pages[pages.len() / 2] as u32;
        }
    }
    total_middle_of_correct
}

mod part1 {
    use super::*;

    pub fn execute() {
        let input = read_input();

        let (adjacency_list, updates) = parse_input(&input);
        let total_middle_of_correct = calc_middle_of_correct(&adjacency_list, &updates);
        println!(
            "PART 1 - TOTAL MIDDLE OF CORRECT: {}",
            &total_middle_of_correct
        );
    }
}

mod part2 {
    use super::*;

    pub fn execute() {
        let input = read_input();
        let (adjacency_list, updates) = parse_input(&input);
    }
}
