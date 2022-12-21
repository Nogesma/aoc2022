extern crate core;

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct OpMonkey {
    m1: String,
    op: char,
    m2: String,
}

fn parse_monkeys(input: &str) -> (HashMap<String, i64>, HashMap<String, OpMonkey>) {
    let mut number_monkeys: HashMap<String, i64> = HashMap::new();
    let mut op_monkeys: HashMap<String, OpMonkey> = HashMap::new();

    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .for_each(|line| {
            let (name, t) = line.split_once(':').unwrap();
            let t = t.trim();
            if t.chars().all(|x| x.is_ascii_digit()) {
                number_monkeys.insert(name.to_string(), t.parse().unwrap());
            } else {
                let mut op = t.split_whitespace();
                op_monkeys.insert(
                    name.to_string(),
                    OpMonkey {
                        m1: op.next().unwrap().to_string(),
                        op: op.next().unwrap().chars().next().unwrap(),
                        m2: op.next().unwrap().to_string(),
                    },
                );
            }
        });

    (number_monkeys, op_monkeys)
}

fn apply_op(a: i64, op: &char, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '/' => a / b,
        '*' => a * b,
        _ => panic!(),
    }
}

fn get_branch_result(
    mnk: &str,
    number_monkeys: &HashMap<String, i64>,
    op_monkeys: &HashMap<String, OpMonkey>,
) -> i64 {
    if let Some(&number) = number_monkeys.get(mnk) {
        number
    } else if let Some(OpMonkey { m1, op, m2 }) = op_monkeys.get(mnk) {
        let m1_number = get_branch_result(m1, number_monkeys, op_monkeys);
        if m1_number == -1 {
            return -1;
        }

        let m2_number = get_branch_result(m2, number_monkeys, op_monkeys);
        if m2_number == -1 {
            return -1;
        }

        apply_op(m1_number, op, m2_number)
    } else {
        panic!();
    }
}

fn reverse_op(a: i64, op: &char, b: i64, res: i64) -> Option<i64> {
    if a == -1 {
        match op {
            '+' => Some(res - b),
            '-' => Some(res + b),
            '/' => Some(res * b),
            '*' => Some(res / b),
            '=' => Some(b),
            _ => None,
        }
    } else {
        match op {
            '+' => Some(res - a),
            '-' => Some(a - res),
            '/' => Some(a / res),
            '*' => Some(res / a),
            '=' => Some(a),
            _ => None,
        }
    }
}

fn get_humn_number(
    mnk: &str,
    number_monkeys: &HashMap<String, i64>,
    op_monkeys: &HashMap<String, OpMonkey>,
    number: i64,
) -> i64 {
    if mnk == "humn" {
        return number;
    }
    if let Some(OpMonkey { m1, op, m2 }) = op_monkeys.get(mnk) {
        let m1_number = get_branch_result(m1, number_monkeys, op_monkeys);
        let m2_number = get_branch_result(m2, number_monkeys, op_monkeys);

        let number = reverse_op(m1_number, op, m2_number, number).unwrap();
        if m1_number == -1 {
            get_humn_number(m1, number_monkeys, op_monkeys, number)
        } else {
            get_humn_number(m2, number_monkeys, op_monkeys, number)
        }
    } else {
        panic!()
    }
}

pub fn main() {
    let input = &fs::read_to_string("day21/input").unwrap();

    let (mut number_monkeys, mut op_monkeys) = parse_monkeys(input);

    println!(
        "Part 1: {}",
        get_branch_result("root", &number_monkeys, &op_monkeys)
    );

    if let Some(root) = op_monkeys.get_mut("root") {
        root.op = '='
    } else {
        panic!()
    };

    if let Some(humn) = number_monkeys.get_mut("humn") {
        *humn = -1;
    } else {
        panic!()
    };

    println!(
        "Part 2: {}",
        get_humn_number("root", &number_monkeys, &op_monkeys, -1)
    );
}
