extern crate core;

use std::fs;

fn has_overlap(line: &str) -> Option<(i32, i32)> {
    let v: [[i32; 2]; 2] = line
        .split(',')
        .filter_map(|x| {
            x.split('-')
                .filter_map(|y| y.parse::<i32>().ok())
                .collect::<Vec<i32>>()
                .try_into()
                .ok()
        })
        .collect::<Vec<[i32; 2]>>()
        .try_into()
        .ok()?;

    Some((
        ((v[0][0] <= v[1][0] && v[0][1] >= v[1][1]) || v[1][0] <= v[0][0] && v[1][1] >= v[0][1])
            as i32,
        ((v[1][0] <= v[0][1] && v[0][0] <= v[1][0]) || (v[0][0] <= v[1][1] && v[1][0] <= v[0][0]))
            as i32,
    ))
}

fn get_total_overlap(input: &str) -> (i32, i32) {
    input
        .split('\n')
        .filter_map(has_overlap)
        .fold((0, 0), |(a, b), (x, y)| (a + x, b + y))
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let res = get_total_overlap(input);
    println!("Part 1: {}", res.0);
    println!("Part 2: {}", res.1);
}
