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

fn get_common_item(
    e1: Result<String, std::io::Error>,
    e2: Result<String, std::io::Error>,
    e3: Result<String, std::io::Error>,
) -> i32 {
    let e1 = e1.unwrap();
    let e2 = e2.unwrap();
    let e3 = e3.unwrap();

    for c in e1.chars() {
        if e2.contains(c) && e3.contains(c) {
            if c.is_uppercase() {
                return c as i32 - 'A' as i32 + 27;
            }
            return c as i32 - 'a' as i32 + 1;
        }
    }
    panic!("No common chars:\n{}\n{}\n{}", e1, e2, e3);
}

fn get_total_score(input: &mut Enumerate<Lines<BufReader<File>>>) -> i32 {
    let mut total = 0;

    loop {
        let e1 = input.next();
        if let None = e1 {
            return total;
        }
        let e2 = input.next();
        let e3 = input.next();

        if let None = e2 {
            panic!("Missing second elf");
        }
        if let None = e3 {
            panic!("Missing third elf");
        }

        total += get_common_item(e1.unwrap().1, e2.unwrap().1, e3.unwrap().1);
    }
}

fn main() {
    let mut input = get_input();
    println!("{}", get_total_score(&mut input));
}
