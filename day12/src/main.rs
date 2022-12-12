extern crate core;

use std::fs;

#[derive(Debug, Clone)]
struct Point {
    height: i32,
    depth: i32,
}

fn get_map(input: &str) -> Result<(Vec<Vec<Point>>, [usize; 2]), &'static str> {
    let mut end: Option<[usize; 2]> = None;

    let map = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => Point {
                        height: 0,
                        depth: i32::MAX,
                    },
                    'E' => {
                        end = Some([y, x]);
                        Point {
                            height: 'z' as i32 - 'a' as i32 + 2,
                            depth: 0,
                        }
                    }
                    _ => Point {
                        height: c as i32 - 'a' as i32 + 1,
                        depth: i32::MAX,
                    },
                })
                .collect()
        })
        .collect();

    if let Some(pos) = end {
        Ok((map, pos))
    } else {
        Err("No end in map.")
    }
}

fn calculate_next(
    map: &mut [Vec<Point>],
    [y, x]: [usize; 2],
    [a, b]: [usize; 2],
    count: &mut i32,
) -> bool {
    let current = &map[y][x];
    let next = &map[a][b];

    if current.height <= next.height + 1 {
        *count += 1;
        if next.depth == i32::MAX {
            map[a][b].depth = current.depth + 1;
            return true;
        }
    }
    false
}

fn navigate_point(map: &mut Vec<Vec<Point>>, [y, x]: [usize; 2]) -> Vec<[usize; 2]> {
    let mut next: Vec<[usize; 2]> = Vec::with_capacity(4);

    if map[y][x].depth == -1 {
        return next;
    }

    let height = map.len();
    let width = map[0].len();
    let mut count = 0;

    let mut f = |[a, b]: [usize; 2]| calculate_next(map, [y, x], [a, b], &mut count);
    // Up
    if y > 0 && f([y - 1, x]) {
        next.push([y - 1, x]);
    }
    // Down
    if y < height - 1 && f([y + 1, x]) {
        next.push([y + 1, x]);
    }
    // Left
    if x > 0 && f([y, x - 1]) {
        next.push([y, x - 1]);
    }
    // Right
    if x < width - 1 && f([y, x + 1]) {
        next.push([y, x + 1]);
    }

    if count == 0 {
        map[y][x].depth = -1;
    }

    next
}

fn navigate_map(map: &mut Vec<Vec<Point>>, pos: [usize; 2]) -> Result<(i32, i32), &'static str> {
    let len = map.len();

    let mut curr = Vec::with_capacity(len);
    let mut next = Vec::with_capacity(len);

    curr.push(pos);

    let mut height = 1;
    let mut p2: i32 = 0;

    while !curr.is_empty() {
        for p in curr {
            if map[p[0]][p[1]].height == height {
                if height == 1 {
                    p2 = map[p[0]][p[1]].depth;
                    height = 0;
                } else {
                    return Ok((map[p[0]][p[1]].depth, p2));
                }
            }
            next.append(&mut navigate_point(map, p));
        }
        curr = next.clone();
        next.clear();
    }
    Err("No path found.")
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let (mut map, end) = get_map(input).unwrap();

    let (p1, p2) = navigate_map(&mut map, end).unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
