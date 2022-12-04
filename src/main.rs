extern crate core;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;

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

fn has_overlap((_, line): (usize, Result<String, std::io::Error>)) -> i32 {
    let line = line.unwrap();

    let v: [[i32; 2]; 2] = line
        .split(',')
        .map(|x| {
            x.split('-')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[i32; 2]>>()
        .try_into()
        .unwrap();

    ((v[0][0] <= v[1][0] && v[0][1] >= v[1][1]) || v[1][0] <= v[0][0] && v[1][1] >= v[0][1]) as i32
}

fn get_total_overlap(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    input.map(has_overlap).sum()
}

fn main() {
    let mut input = get_input();
    println!("{}", get_total_overlap(&mut input));
}
