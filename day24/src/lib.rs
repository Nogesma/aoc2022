extern crate core;

use crate::Direction::{East, North, South, West};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::swap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    West,
    South,
    East,
    North,
}

fn bfs(
    blizzards: &HashMap<(usize, usize), Vec<Direction>>,
    (height, width): &(usize, usize),
    pos: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let mut next = HashSet::with_capacity(5);

    // North
    if pos.0 != 0 && (pos.1 == 1 || pos.0 > 1) && !blizzards.contains_key(&(pos.0 - 1, pos.1)) {
        next.insert((pos.0 - 1, pos.1));
    }

    // South
    if pos.0 != height - 1
        && (pos.1 == width - 2 || pos.0 < height - 2)
        && !blizzards.contains_key(&(pos.0 + 1, pos.1))
    {
        next.insert((pos.0 + 1, pos.1));
    }

    // East
    if pos.0 != 0 && pos.1 != width - 2 && !blizzards.contains_key(&(pos.0, pos.1 + 1)) {
        next.insert((pos.0, pos.1 + 1));
    }

    // West
    if pos.0 != height - 1 && pos.1 != 1 && !blizzards.contains_key(&(pos.0, pos.1 - 1)) {
        next.insert((pos.0, pos.1 - 1));
    }

    // Wait
    if !blizzards.contains_key(pos) {
        next.insert(*pos);
    }

    next
}

fn move_blizzards(
    blizzards: &mut HashMap<(usize, usize), Vec<Direction>>,
    cpy: &mut HashMap<(usize, usize), Vec<Direction>>,
    (height, width): &(usize, usize),
) {
    assert!(cpy.is_empty());
    for (pos, dir) in blizzards.iter() {
        for direction in dir {
            let next = match direction {
                North => {
                    if pos.0 == 1 {
                        (height - 2, pos.1)
                    } else {
                        (pos.0 - 1, pos.1)
                    }
                }
                South => {
                    if pos.0 == height - 2 {
                        (1, pos.1)
                    } else {
                        (pos.0 + 1, pos.1)
                    }
                }
                East => {
                    if pos.1 == width - 2 {
                        (pos.0, 1)
                    } else {
                        (pos.0, pos.1 + 1)
                    }
                }
                West => {
                    if pos.1 == 1 {
                        (pos.0, width - 2)
                    } else {
                        (pos.0, pos.1 - 1)
                    }
                }
            };
            if let Some(pos) = cpy.get_mut(&next) {
                pos.push(*direction);
            } else {
                cpy.insert(next, vec![*direction]);
            }
        }
    }
    swap(cpy, blizzards);
    cpy.clear();
}

fn navigate_valley(
    blizzards: &mut HashMap<(usize, usize), Vec<Direction>>,
    size: &(usize, usize),
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let mut curr = HashSet::new();
    let mut next = HashSet::new();

    let mut blizzard_cpy = HashMap::with_capacity(blizzards.len());
    curr.insert(start);

    let mut t = 0;
    loop {
        move_blizzards(blizzards, &mut blizzard_cpy, size);
        for state in &curr {
            next.extend(bfs(blizzards, size, state).iter());
        }

        t += 1;
        if next.contains(&end) {
            return t;
        }
        swap(&mut curr, &mut next);
        next.clear();
    }
}

fn _print_map(
    blizzards: &HashMap<(usize, usize), Vec<Direction>>,
    (height, width): (usize, usize),
    player: (usize, usize),
) {
    for x in 0..height {
        for y in 0..width {
            let c = if player == (x, y) {
                'E'
            } else if let Some(blizzard) = blizzards.get(&(x, y)) {
                if blizzard.len() == 2 {
                    '2'
                } else if blizzard.len() == 3 {
                    '3'
                } else if blizzard.len() == 4 {
                    '4'
                } else {
                    match blizzard[0] {
                        North => '^',
                        South => 'v',
                        East => '>',
                        West => '<',
                    }
                }
            } else if x == height - 1 && y == width - 2 {
                '.'
            } else if x == 0 || x == height - 1 || y == 0 || y == width - 1 {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn main() {
    let input = &fs::read_to_string("day24/input").unwrap();

    let mut lines = input.split('\n').filter(|x| !x.is_empty());

    let mut blizzards: HashMap<(usize, usize), Vec<Direction>> = lines
        .clone()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| match c {
                '>' => Some(((x, y), vec![East])),
                '<' => Some(((x, y), vec![West])),
                '^' => Some(((x, y), vec![North])),
                'v' => Some(((x, y), vec![South])),
                _ => None,
            })
        })
        .collect();

    let height = lines.clone().count();
    let width = lines.next().unwrap().len();

    let p1 = navigate_valley(
        &mut blizzards,
        &(height, width),
        (0, 1),
        (height - 1, width - 2),
    );

    println!("Part 1: {}", p1);

    println!(
        "Part 2: {}",
        p1 + navigate_valley(
            &mut blizzards,
            &(height, width),
            (height - 1, width - 2),
            (0, 1),
        ) + navigate_valley(
            &mut blizzards,
            &(height, width),
            (0, 1),
            (height - 1, width - 2),
        )
    );
}
