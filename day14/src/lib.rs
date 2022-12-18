extern crate core;

use std::cmp::max;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
enum Materials {
    Air,
    Rock,
    Sand,
}

fn parse_pos(input: Option<&str>) -> Option<(usize, usize)> {
    let (a, b) = input?.split_once(',')?;

    Some((a.parse().ok()?, b.parse().ok()?))
}

fn set_min_height(map: &mut Vec<Vec<Materials>>, size: usize) {
    while map.len() <= size {
        map.push(Vec::new());
    }
}

fn set_min_width(map: &mut Vec<Materials>, size: usize) {
    while map.len() <= size {
        map.push(Materials::Air);
    }
}

fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

fn parse_map(input: &str) -> Vec<Vec<Materials>> {
    let lines = input.split('\n');
    let mut map = vec![vec![]];

    for line in lines {
        if line.is_empty() {
            return map;
        }
        let mut pos: Vec<&str> = line.split(" -> ").collect();

        let mut curr = parse_pos(pos.pop()).unwrap();
        while !pos.is_empty() {
            let next = parse_pos(pos.pop()).unwrap();

            for i in range_inclusive(curr.1, next.1) {
                set_min_height(&mut map, i);
                for j in range_inclusive(curr.0, next.0) {
                    set_min_width(&mut map[i], j);
                    map[i][j] = Materials::Rock;
                }
            }
            curr = next;
        }
    }

    Vec::new()
}

fn normalize_map(map: &mut Vec<Vec<Materials>>) {
    let max = map.iter().fold(0, |accum, v| max(accum, v.len())) * 3 / 2;

    map.push(Vec::new());
    map.push(Vec::new());

    for v in &mut *map {
        set_min_width(v, max);
    }

    let len = map.len() - 1;
    for i in 0..map[len].len() {
        map[len][i] = Materials::Rock;
    }
}

fn _print_map(map: &Vec<Vec<Materials>>) {
    sleep(Duration::from_millis(50));
    print!("{}[2J", 27 as char);
    for i in map {
        for j in i {
            match j {
                Materials::Air => print!("."),
                Materials::Rock => print!("#"),
                Materials::Sand => print!("o"),
            }
        }
        println!();
    }
}

fn drop_sand(map: &mut Vec<Vec<Materials>>) -> (i32, i32) {
    let mut p1 = 0;
    loop {
        let (mut i, mut j) = (0, 500);

        if map[i][j] == Materials::Sand {
            return (p1, count_sand(map));
        }

        map[i][j] = Materials::Sand;
        let mut is_stopped = false;
        while !is_stopped {
            // _print_map(map);
            if i == map.len() - 2 && p1 == 0 {
                p1 = count_sand(map) - 1;
            }

            if map[i + 1][j] == Materials::Air {
                map[i][j] = Materials::Air;
                i += 1;
                map[i][j] = Materials::Sand;
            } else if map[i + 1][j - 1] == Materials::Air {
                map[i][j] = Materials::Air;
                i += 1;
                j -= 1;
                map[i][j] = Materials::Sand;
            } else if map[i + 1][j + 1] == Materials::Air {
                map[i][j] = Materials::Air;
                i += 1;
                j += 1;
                map[i][j] = Materials::Sand;
            } else {
                is_stopped = true;
            }
        }
    }
}

fn count_sand(map: &[Vec<Materials>]) -> i32 {
    map.iter().fold(0, |accum, v| {
        accum
            + v.iter()
                .fold(0, |a, m| if *m == Materials::Sand { a + 1 } else { a })
    })
}

pub fn main() {
    let input = &fs::read_to_string("day14/input").unwrap();

    let mut map = parse_map(input);

    normalize_map(&mut map);

    let (p1, p2) = drop_sand(&mut map);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
