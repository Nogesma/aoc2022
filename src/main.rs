extern crate core;

use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
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

fn get_calories(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    let mut cals = 0;
    for (_, line) in input {
        let line = line.unwrap();
        if line.len() == 0 {
            return cals;
        }
        cals += line.parse::<i32>().unwrap();
    }
    return -1;
}

fn get_max_calories(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    let mut max_cals = 0;
    let mut cals: i32 = 0;
    while cals != -1 {
        cals = get_calories(input);
        if cals > max_cals {
            max_cals = cals;
        }
    }
    return max_cals;
}

fn main() {
    let mut input = get_input();
    println!("{}", get_max_calories(&mut input));
}
