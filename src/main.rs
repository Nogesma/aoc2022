extern crate core;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;

fn get_input() -> Enumerate<Lines<BufReader<File>>> {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    return reader.lines().enumerate();
}

fn get_shape_score(a: i32, b: i32) -> i32 {
    if b == 1 {
        return a;
    }
    return (if b == 0 { a - 1 } else { a + 1 }).rem_euclid(3);
}

fn get_round_score((_, line): (usize, Result<String, std::io::Error>)) -> i32 {
    let line = line.unwrap();

    let a = line.chars().next().unwrap() as i32 - 'A' as i32;
    let b = line.chars().last().unwrap() as i32 - 'X' as i32;

    return get_shape_score(a, b) + 1 + b * 3;
}

fn get_total_score(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    return input.map(get_round_score).sum();
}

fn main() {
    let mut input = get_input();
    println!("{}", get_total_score(&mut input));
}
