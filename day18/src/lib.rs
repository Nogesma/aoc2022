extern crate core;

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn into_point(mut iter: impl Iterator<Item = i32>) -> Option<Point> {
    Some(Point {
        x: iter.next()?,
        y: iter.next()?,
        z: iter.next()?,
    })
}

fn get_cube_bounds(cube: &HashSet<Point>) -> Option<[Point; 2]> {
    let x_coords = cube.iter().map(|v| v.x);
    let y_coords = cube.iter().map(|v| v.y);
    let z_coords = cube.iter().map(|v| v.z);

    Some([
        Point {
            x: x_coords.clone().min()?,
            y: y_coords.clone().min()?,
            z: z_coords.clone().min()?,
        },
        Point {
            x: x_coords.max()?,
            y: y_coords.max()?,
            z: z_coords.max()?,
        },
    ])
}

fn get_adjacent_points(Point { x, y, z }: Point) -> Box<[Point; 6]> {
    Box::new([
        Point { x: x - 1, y, z },
        Point { x, y: y - 1, z },
        Point { x, y, z: z - 1 },
        Point { x: x + 1, y, z },
        Point { x, y: y + 1, z },
        Point { x, y, z: z + 1 },
    ])
}

fn dfs(cur: Point, goal: &[Point; 2], lava: &HashSet<Point>, visited: &mut HashSet<Point>) -> i32 {
    if visited.contains(&cur) {
        return 0;
    }
    if lava.contains(&cur) {
        return 1;
    }

    if cur.x < goal[0].x - 1
        || cur.y < goal[0].y - 1
        || cur.z < goal[0].z - 1
        || cur.x > goal[1].x + 1
        || cur.y > goal[1].y + 1
        || cur.z > goal[1].z + 1
    {
        return 0;
    }
    visited.insert(cur);

    let points = get_adjacent_points(cur);

    let mut sum = 0;
    for point in *points {
        sum += dfs(point, goal, lava, visited);
    }
    sum
}

fn count_adjacents(lava: &HashSet<Point>) -> usize {
    let mut sum = 0;

    for point in lava {
        let points = get_adjacent_points(*point);
        for adjacent in *points {
            if lava.contains(&adjacent) {
                sum += 1;
            }
        }
    }
    sum
}

pub fn main() {
    let input = &fs::read_to_string("day18/input").unwrap();

    let lava: HashSet<Point> = input
        .split('\n')
        .filter_map(|v| into_point(v.split(',').filter_map(|x| x.parse::<i32>().ok())))
        .collect();

    let sum = lava.len() * 6 - count_adjacents(&lava);

    println!("Part 1: {}", sum);

    let cube = get_cube_bounds(&lava).unwrap();
    let mut free: HashSet<Point> = HashSet::new();

    println!("Part 2: {}", dfs(cube[0], &cube, &lava, &mut free));
}
