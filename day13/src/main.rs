extern crate core;

use std::cmp::{min, Ordering};
use std::fs;

use json::JsonValue;
use json::JsonValue::Array;

fn comp(a: &JsonValue, b: &JsonValue) -> Ordering {
    for i in 0..(min(a.len(), b.len())) {
        // println!("{:?}\n{:?}\n", a[i], b[i]);
        match (&a[i], &b[i]) {
            (a, b) if a.is_number() && b.is_number() => match a.as_i32().cmp(&b.as_i32()) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            },
            (a, b) if a.is_array() && b.is_array() => match comp(a, b) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            },
            (a, b) if a.is_array() && b.is_number() => match comp(a, &Array(vec![b.clone()])) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            },
            (a, b) if a.is_number() && b.is_array() => match comp(&Array(vec![a.clone()]), b) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            },
            _ => panic!("Could not find match in comp."),
        }
    }
    a.len().cmp(&b.len())
}

fn is_pair_ordered(pair: &&str) -> usize {
    let (first, last) = pair.split_once('\n').unwrap();

    let first = json::parse(first).unwrap();
    let last = json::parse(last).unwrap();

    match comp(&first, &last) {
        Ordering::Greater => 0,
        Ordering::Less => 1,
        Ordering::Equal => panic!("Values are equal."),
    }
}

fn p1(input: &str) -> usize {
    let pairs: Vec<&str> = input.split("\n\n").collect();

    let a = pairs.iter().map(is_pair_ordered);
    a.enumerate().fold(
        0,
        |accum, (i, v)| if v != 0 { accum + i + 1 } else { accum },
    )
}

fn p2(input: &str) -> usize {
    let mut packets = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .filter_map(|line| json::parse(line).ok())
        .collect::<Vec<JsonValue>>();

    packets.push(json::parse("[[2]]").unwrap());
    packets.push(json::parse("[[6]]").unwrap());

    packets.sort_by(comp);

    packets.iter().enumerate().fold(1, |accum, (i, v)| {
        let s = json::stringify(v.clone());
        if s == "[[2]]" || s == "[[6]]" {
            accum * (i + 1)
        } else {
            accum
        }
    })
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}
