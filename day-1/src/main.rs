extern crate core;

use std::fs;

fn get_max_calories(input: &str) -> [i32; 3] {
    let mut max_cals = [0, 0, 0];
    let mut cals = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            if cals > max_cals[0] {
                max_cals[0] = cals;
            }
            max_cals.sort();
            cals = 0;
        } else {
            cals += line.parse::<i32>().unwrap();
        }
    }
    max_cals
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let cals = get_max_calories(input);
    println!("Part 1: {}", cals.last().unwrap());
    println!("Part 2: {}", cals.iter().sum::<i32>());
}
