extern crate core;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: (char, u64),
    test: [usize; 3],
    inspect: u64,
}

fn get_last_number(str: &str) -> Result<usize, std::num::ParseIntError> {
    let (_, split) = str.rsplit_once(' ').unwrap();

    split.parse()
}

fn get_monkeys(input: &str) -> Result<Vec<Monkey>, &'static str> {
    let mut lines = input.split('\n');
    let mut monkeys = Vec::new();

    loop {
        let x = lines.next();
        if x.is_none() {
            return Ok(monkeys);
        }
        let mut line = lines.next().unwrap();
        let (_, items) = line.split_once(':').unwrap();
        let items = items
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect::<VecDeque<u64>>();

        line = lines.next().unwrap();
        let op = line.split_at(line.find("old ").unwrap() + 4).1;
        let operation = op.split_once(' ').unwrap();
        let operation = (
            operation.0.chars().next().unwrap(),
            operation.1.parse::<u64>().unwrap_or_default(),
        );

        let test: [usize; 3] = (0..3)
            .map(|_| {
                line = lines.next().unwrap();
                get_last_number(line).unwrap()
            })
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            inspect: 0,
        });
        lines.next().unwrap();
    }
}

fn apply_op(item: u64, (op, x): (char, u64)) -> Option<u64> {
    let v = if x == 0 { item } else { x };
    match op {
        '+' => Some(item + v),
        '*' => Some(item * v),
        _ => None,
    }
}

fn test(worry: u64, [div, m1, m2]: [usize; 3]) -> usize {
    if worry as usize % div == 0 {
        m1
    } else {
        m2
    }
}

fn worry_a_lot(monkeys: &mut [Monkey], modulo: u64) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let item = monkeys[i].items.pop_front().unwrap();
            monkeys[i].inspect += 1;

            let worry = if modulo != 0 {
                apply_op(item, monkeys[i].operation).unwrap() % modulo
            } else {
                apply_op(item, monkeys[i].operation).unwrap() / 3
            };
            let rec = test(worry, monkeys[i].test);
            monkeys[rec].items.push_back(worry);
        }
    }
}

fn get_inspections(monkeys: &[Monkey]) -> u64 {
    let mut inspections = monkeys.iter().map(|x| x.inspect).collect::<Vec<u64>>();
    inspections.sort();

    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn throw_items(monkeys: Vec<Monkey>, modulo: u64) -> (u64, u64) {
    let mut p1 = monkeys.clone();
    let mut p2 = monkeys;
    for u in 0..10_000 {
        if u < 20 {
            worry_a_lot(&mut p1, 0);
        }
        worry_a_lot(&mut p2, modulo);
    }

    (get_inspections(&p1), get_inspections(&p2))
}

pub fn main() {
    let input = &fs::read_to_string("day11/input").unwrap();

    let monkeys: Vec<Monkey> = get_monkeys(input).unwrap();

    let modulo = monkeys
        .iter()
        .fold(1, |accum, v| *v.test.first().unwrap() as u64 * accum);

    let (r1, r2) = throw_items(monkeys, modulo);

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
