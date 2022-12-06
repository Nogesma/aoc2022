extern crate core;

use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;

const SIZE: usize = 4;

fn get_input() -> Enumerate<Lines<BufReader<File>>> {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    reader.lines().enumerate()
}

fn check_duplicates(v: VecDeque<char>) -> bool {
    v.iter().collect::<HashSet<&char>>().len() == SIZE
}

fn parse_stack((_, line): (usize, Result<String, std::io::Error>)) -> Result<usize, &'static str> {
    let mut line = line.unwrap();

    let mut v: VecDeque<char> = line.drain(0..SIZE).collect();
    let mut res = SIZE;

    for c in line.chars() {
        res += 1;
        v.rotate_left(1);
        v[SIZE - 1] = c;
        if check_duplicates(v.clone()) {
            return Ok(res);
        }
    }

    Err("No start of packet marker.")
}

fn main() {
    let mut input = get_input();

    println!("{}", parse_stack(input.next().unwrap()).unwrap());
}
