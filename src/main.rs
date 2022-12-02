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

fn get_outcome_score(a: u32, b: u32) -> u32 {
    if a == b {
        return 3;
    }
    return if a > b {
        if b == 1 && a == 3 {
            return 6;
        }
        0
    } else {
        if a == 1 && b == 3 {
            return 0;
        }
        6
    };
}

fn get_round_score((_, line): (usize, Result<String, std::io::Error>)) -> u32 {
    let line = line.unwrap();

    let a = line.chars().next().unwrap() as u32 - 'A' as u32 + 1;
    let b = line.chars().last().unwrap() as u32 - 'X' as u32 + 1;

    let outcome_score = get_outcome_score(a, b);

    return outcome_score + b;
}

fn get_total_score(input: &mut Enumerate<Lines<BufReader<File>>>) -> u32 {
    return input.map(get_round_score).sum();
}

fn main() {
    let mut input = get_input();
    println!("{}", get_total_score(&mut input));
}
