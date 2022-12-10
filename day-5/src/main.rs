extern crate core;
use std::fs;

fn parse_stack(input: &str) -> (Vec<Vec<char>>, Vec<usize>) {
    let mut input = input.split('\n').rev();
    let line = input.next().unwrap();

    let size = (line.len() + 2) / 4;

    let mut stack: Vec<Vec<char>> = (0..size).map(|_| Vec::new()).collect();

    for line in input {
        line.chars()
            .skip(1)
            .enumerate()
            .step_by(4)
            .for_each(|(s, c)| {
                if c.is_uppercase() {
                    stack[s / 4].push(c)
                }
            });
    }

    let stack_len = stack.iter().map(|x| x.len()).collect::<Vec<usize>>();

    stack.iter_mut().for_each(|x| {
        x.resize(stack_len.iter().sum(), '\0');
    });

    (stack, stack_len)
}

fn apply_move(stack: &mut [Vec<char>], len: &mut [usize], line: &str, p1: bool) {
    let y = line
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok());

    let y: [usize; 3] = y.map(|x| x - 1).collect::<Vec<usize>>().try_into().unwrap();

    let i = len[y[1]];
    let j = len[y[2]];

    if p1 {
        stack[y[1]][i - y[0] - 1..i].reverse();
    }

    let e = stack[y[1]][i - y[0] - 1..i].to_vec();

    stack[y[2]][j..j + y[0] + 1].copy_from_slice(e.as_slice());

    len[y[1]] -= y[0] + 1;
    len[y[2]] += y[0] + 1;
}

fn move_stack(input: &str, stack: &mut [Vec<char>], len: &mut [usize], p1: bool) -> String {
    let input = input.split('\n');

    for s in input.filter(|x| !x.is_empty()) {
        apply_move(stack, len, s, p1);
    }

    stack
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| {
            if v[len[idx] - 1].is_alphabetic() {
                Some(v[len[idx] - 1])
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let file = &fs::read_to_string("input").unwrap();

    let (stack, moves) = file.split_once("\n\n").unwrap();

    let (stack, len) = parse_stack(stack);

    println!(
        "Part 1: {}",
        move_stack(moves, &mut stack.clone(), &mut len.clone(), true)
    );
    println!(
        "Part 2: {}",
        move_stack(moves, &mut stack.clone(), &mut len.clone(), false)
    );
}
