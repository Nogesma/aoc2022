extern crate core;

use nalgebra::Point2;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

/* Part I */
// const SIZE: usize = 2;

/* Part II */
const SIZE: usize = 10;

fn is_touching(head: &Point2<i32>, tail: &Point2<i32>) -> bool {
    (((tail.x - head.x).pow(2) + (tail.y - head.y).pow(2)) as f32).sqrt() < 2.0
}

fn follow_head(knots: &mut [Point2<i32>; SIZE]) {
    for i in 0..SIZE - 1 {
        if is_touching(&knots[i], &knots[i + 1]) {
            return;
        }

        match knots[i + 1].x.cmp(&knots[i].x) {
            Ordering::Greater => knots[i + 1].x -= 1,
            Ordering::Less => knots[i + 1].x += 1,
            Ordering::Equal => {}
        }

        match knots[i + 1].y.cmp(&knots[i].y) {
            Ordering::Greater => knots[i + 1].y -= 1,
            Ordering::Less => knots[i + 1].y += 1,
            Ordering::Equal => {}
        }
    }
}

fn apply_move(m: &str, knots: &mut [Point2<i32>; SIZE]) -> Result<(), &'static str> {
    match m {
        "U" => knots[0].y += 1,
        "R" => knots[0].x += 1,
        "L" => knots[0].x -= 1,
        "D" => knots[0].y -= 1,
        _ => return Err("Move not recognised"),
    }

    follow_head(knots);

    Ok(())
}

fn part_1(input: &str) -> usize {
    let mut knots: [Point2<i32>; SIZE] = [Point2::origin(); SIZE];

    let mut visited: HashSet<Point2<i32>> = HashSet::new();
    visited.insert(*knots.last().unwrap());

    let lines = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    for line in lines {
        let (m, n) = line.split_once(' ').unwrap();

        for _ in 0..n.parse().unwrap() {
            apply_move(m, &mut knots).unwrap();
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    println!("{}", part_1(input));
}
