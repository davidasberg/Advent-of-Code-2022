use std::collections::HashSet;
use std::fs;

fn read_input(file: &str) -> Vec<char> {
    let input = fs::read_to_string(file).expect("Something went wrong reading the file");
    input.chars().collect()
}

pub fn find_first_marker(message: Vec<char>, window_size: usize) -> usize {
    message
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() == window_size)
        .unwrap()
        .0
        + window_size
}

pub fn part1() {
    let message = read_input("input/day06.in");
    println!("{}", find_first_marker(message, 4));
}

pub fn part2() {
    let message = read_input("input/day06.in");
    println!("{}", find_first_marker(message, 14));
}
