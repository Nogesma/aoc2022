extern crate core;

use crate::Direction::{East, North, South, West};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Direction {
    West,
    South,
    East,
    North,
}

fn has_elf(elves: &HashMap<(i32, i32), Option<(i32, i32)>>, pos: &[(i32, i32); 8]) -> u8 {
    let mut res: u8 = 0b00000000;
    for (i, v) in pos.iter().enumerate() {
        if elves.contains_key(v) {
            res |= 1 << (7 - i);
        }
    }
    res
}

fn start_round(elves: &mut HashMap<(i32, i32), Option<(i32, i32)>>, directions: &[Direction; 4]) {
    let elfcpy = elves.clone();
    for (pos, next) in elves.iter_mut() {
        let adjacents = [
            (pos.0 - 1, pos.1 - 1),
            (pos.0 - 1, pos.1),
            (pos.0 - 1, pos.1 + 1),
            (pos.0, pos.1 + 1),
            (pos.0 + 1, pos.1 + 1),
            (pos.0 + 1, pos.1),
            (pos.0 + 1, pos.1 - 1),
            (pos.0, pos.1 - 1),
        ];
        let adjacent_pos = has_elf(&elfcpy, &adjacents);

        *next = None;
        if adjacent_pos != 0 {
            for direction in directions {
                if match direction {
                    North => adjacent_pos & 0b11100000 == 0,
                    East => adjacent_pos & 0b00111000 == 0,
                    South => adjacent_pos & 0b00001110 == 0,
                    West => adjacent_pos & 0b10000011 == 0,
                } {
                    *next = Some(match direction {
                        North => (pos.0 - 1, pos.1),
                        East => (pos.0, pos.1 + 1),
                        South => (pos.0 + 1, pos.1),
                        West => (pos.0, pos.1 - 1),
                    });

                    break;
                }
            }
        }
    }
}

fn end_round(elves: &mut HashMap<(i32, i32), Option<(i32, i32)>>) {
    let mut next_pos: HashMap<(i32, i32), (i32, i32, bool)> = HashMap::with_capacity(elves.len());
    for (pos, next) in elves.iter() {
        if let Some(next) = next {
            let i = next_pos.get_mut(next);
            if let Some(i) = i {
                i.2 = true;
            } else {
                next_pos.insert(*next, (pos.0, pos.1, false));
            }
        }
    }

    for next in next_pos {
        if !next.1 .2 {
            let pos = (next.1 .0, next.1 .1);
            elves.remove(&pos);
            elves.insert(next.0, None);
        }
    }
}

fn p1(elves: &mut HashMap<(i32, i32), Option<(i32, i32)>>, directions: &mut [Direction; 4]) -> i32 {
    for _ in 0..10 {
        start_round(elves, directions);
        end_round(elves);
        directions.rotate_left(1);
    }

    let corners = elves.iter().fold(
        [(i32::MAX, i32::MAX), (i32::MIN, i32::MIN)],
        |mut accum, (pos, _)| {
            if pos.0 < accum[0].0 {
                accum[0].0 = pos.0
            }
            if pos.0 > accum[1].0 {
                accum[1].0 = pos.0
            }
            if pos.1 < accum[0].1 {
                accum[0].1 = pos.1
            }
            if pos.1 > accum[1].1 {
                accum[1].1 = pos.1
            }

            accum
        },
    );

    let mut empty = 0;
    for x in corners[0].0..=corners[1].0 {
        for y in corners[0].1..=corners[1].1 {
            if !elves.contains_key(&(x, y)) {
                empty += 1;
            }
        }
    }
    empty
}

fn p2(elves: &mut HashMap<(i32, i32), Option<(i32, i32)>>, directions: &mut [Direction; 4]) -> i32 {
    let mut j = 0;
    loop {
        start_round(elves, directions);
        j += 1;
        if elves.iter().all(|(_, next)| next.is_none()) {
            return j;
        }
        end_round(elves);
        directions.rotate_left(1);
    }
}

pub fn main() {
    let input = &fs::read_to_string("day23/input").unwrap();

    let mut elves: HashMap<(i32, i32), Option<(i32, i32)>> = input
        .split('\n')
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some(((x as i32, y as i32), None))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut directions = [North, South, West, East];

    println!("Part 1: {}", p1(&mut elves, &mut directions));

    println!("Part 2: {}", 10 + p2(&mut elves, &mut directions));
}
