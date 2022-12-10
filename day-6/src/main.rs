extern crate core;

use std::collections::{HashSet, VecDeque};
use std::fs;

fn check_duplicates(v: &VecDeque<char>, size: usize) -> bool {
    v.iter().collect::<HashSet<&char>>().len() == size
}

fn parse_stack(line: &str, size: usize) -> Option<usize> {
    let mut line = line.to_string();

    let mut v: VecDeque<char> = line.drain(0..size).collect();
    let mut res = size;

    for c in line.chars() {
        res += 1;
        v.rotate_left(1);
        v[size - 1] = c;
        if check_duplicates(&v, size) {
            return Some(res);
        }
    }
    None
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    println!("Part 1: {}", parse_stack(input, 4).unwrap_or_default());
    println!("Part 1: {}", parse_stack(input, 14).unwrap_or_default());
}
