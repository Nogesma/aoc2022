extern crate core;

use std::fs;

fn get_outcome_score(a: i32, b: i32) -> i32 {
    if a == b {
        3
    } else if a > b {
        if b == 1 && a == 3 {
            6
        } else {
            0
        }
    } else if a == 1 && b == 3 {
        0
    } else {
        6
    }
}

fn get_shape_score(a: i32, b: i32) -> i32 {
    if b == 1 {
        a
    } else {
        (if b == 0 { a - 1 } else { a + 1 }).rem_euclid(3)
    }
}

fn get_round_score(line: &str) -> Option<(i32, i32)> {
    let a = line.chars().next()? as i32 - 'A' as i32;
    let b = line.chars().last()? as i32 - 'X' as i32;

    let outcome_score = get_outcome_score(a + 1, b + 1);

    Some((outcome_score + b + 1, get_shape_score(a, b) + 1 + b * 3))
}

fn get_total_score(input: &str) -> (i32, i32) {
    input
        .split('\n')
        .filter_map(get_round_score)
        .fold((0, 0), |(a, b), (x, y)| (a + x, b + y))
}

pub fn day02() {
    let input = &fs::read_to_string("day02/input").unwrap();

    let res = get_total_score(input);
    println!("Part 1: {}", res.0);
    println!("Part 2: {}", res.1);
}
