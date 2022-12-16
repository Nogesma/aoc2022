extern crate core;

use std::fs;

fn get_common_item_in_line(line: &str) -> Option<i32> {
    let (first, second) = line.split_at(line.len() / 2);

    for c in first.chars() {
        if second.contains(c) {
            if c.is_uppercase() {
                return Some(c as i32 - 'A' as i32 + 27);
            }
            return Some(c as i32 - 'a' as i32 + 1);
        }
    }
    None
}

fn p1(input: &str) -> i32 {
    return input.split('\n').filter_map(get_common_item_in_line).sum();
}

fn get_common_3(lines: &mut Vec<&str>) -> Option<i32> {
    let e: [&str; 3] = lines.drain(0..3).as_slice().try_into().ok()?;

    for c in e[0].chars() {
        if e[1].contains(c) && e[2].contains(c) {
            if c.is_uppercase() {
                return Some(c as i32 - 'A' as i32 + 27);
            }
            return Some(c as i32 - 'a' as i32 + 1);
        }
    }
    None
}

fn p2(input: &str) -> i32 {
    let mut total = 0;

    let mut lines = input.split('\n').collect::<Vec<&str>>();
    while lines.len() > 1 {
        total += get_common_3(&mut lines).unwrap_or_default();
    }
    total
}

pub fn day03() {
    let input = &fs::read_to_string("day03/input").unwrap();

    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}
