extern crate core;

use std::collections::VecDeque;
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

fn parse_stack(input: &mut Enumerate<Lines<BufReader<File>>>) -> Vec<VecDeque<char>> {
    let (_, line) = input.next().unwrap();
    let mut line = line.unwrap();

    let size = (line.len() + 1) / 4;

    let mut stack: Vec<VecDeque<char>> = (0..size).map(|_| VecDeque::new()).collect();

    while !line.is_empty() {
        line.chars()
            .skip(1)
            .enumerate()
            .step_by(4)
            .for_each(|(s, c)| {
                if c.is_uppercase() {
                    stack[s / 4].push_back(c)
                }
            });

        line = input.next().unwrap().1.unwrap();
    }
    stack
}

fn apply_move(stack: &mut [VecDeque<char>], (_, line): (usize, Result<String, std::io::Error>)) {
    let line = line.unwrap();

    let y: [usize; 3] = line
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let y = y.map(|x| x - 1);

    for _ in 0..=y[0] {
        let c = stack[y[1]].pop_front().unwrap();
        stack[y[2]].push_front(c);
    }
}

fn move_stack(
    input: &mut Enumerate<Lines<BufReader<File>>>,
    stack: &mut [VecDeque<char>],
) -> String {
    let ap_stack = |x| apply_move(stack, x);

    input.for_each(ap_stack);

    stack.iter().map(|x| x[0]).collect()
}

fn main() {
    let mut input = get_input();

    let mut stack: Vec<VecDeque<char>> = parse_stack(&mut input);
    println!("{}", move_stack(&mut input, &mut stack));
}
