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

fn get_common_item((_, line): (usize, Result<String, std::io::Error>)) -> i32 {
    let line = line.unwrap();

    let (first, second) = line.split_at(line.len() / 2);

    for c in first.chars() {
        if second.contains(c) {
            if c.is_uppercase() {
                return c as i32 - 'A' as i32 + 27;
            }
            return c as i32 - 'a' as i32 + 1;
        }
    }
    panic!("No common chars: {} {}, {}", first, second, line);
}

fn get_total_score(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    return input.map(get_common_item).sum();
}

fn main() {
    let mut input = get_input();
    println!("{}", get_total_score(&mut input));
}
