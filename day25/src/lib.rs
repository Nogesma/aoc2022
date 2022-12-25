extern crate core;

use std::fs;

fn from_snafu(number: &str) -> i64 {
    number
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, v)| match v {
            '0' => 0,
            '1' => 5_i64.pow(idx as u32),
            '2' => 2 * 5_i64.pow(idx as u32),
            '-' => -(5_i64.pow(idx as u32)),
            '=' => -2 * 5_i64.pow(idx as u32),
            _ => panic!(),
        })
        .sum()
}

fn to_snafu(mut number: i64) -> String {
    let mut values = Vec::new();
    let mut res = String::new();

    while number / 5 > 0 {
        values.push(number % 5);
        number /= 5;
    }
    values.push(number % 5);

    for i in 0..values.len() {
        res.push(match values[i] {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                if i + 1 == values.len() {
                    values.push(1);
                } else {
                    values[i + 1] += 1;
                }
                '='
            }
            4 => {
                if i + 1 == values.len() {
                    values.push(1);
                } else {
                    values[i + 1] += 1;
                }
                '-'
            }
            _ => panic!(),
        })
    }

    if res.len() != values.len() {
        res.push(match values.last().unwrap() {
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        })
    }

    res.chars().rev().collect()
}
pub fn main() {
    let input = &fs::read_to_string("day25/input").unwrap();

    let lines = input.split('\n').filter(|x| !x.is_empty());

    println!(
        "Part 1: {:?}",
        to_snafu(lines.map(from_snafu).sum()).as_str()
    )
}
