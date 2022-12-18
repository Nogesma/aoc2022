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

fn dfs(
    cur: Point,
    goal: &[Point; 2],
    lava: &HashSet<Point>,
    visited: &mut HashSet<Point>,
    trapped: &mut HashSet<Point>,
    free: &mut HashSet<Point>,
) -> i8 {
    if free.contains(&cur) {
        free.extend(visited.iter());
        return 1;
    }
    if lava.contains(&cur) {
        return 0;
    }
    if trapped.contains(&cur) {
        return -1;
    }

    if cur.x < goal[0].x
        || cur.y < goal[0].y
        || cur.z < goal[0].z
        || cur.x > goal[1].x
        || cur.y > goal[1].y
        || cur.z > goal[1].z
    {
        free.extend(visited.iter());
        free.insert(cur);
        return 1;
    }

    if visited.contains(&cur) {
        return 0;
    }
    visited.insert(cur);

    let points = get_adjacent_points(cur);

    for pt in *points {
        let res = dfs(pt, goal, lava, visited, trapped, free);
        if res != 0 {
            return res;
        }
    }
    0
}

fn prune_adjacent(cube: &HashSet<Point>, points: &HashSet<Point>) -> usize {
    let mut sum = 0;

    for point in points {
        let points = get_adjacent_points(*point);
        for adjacent in *points {
            if cube.contains(&adjacent) {
                sum += 1;
            }
        }
    }
    sum
}

pub fn main() {
    let input = &fs::read_to_string("day18/input").unwrap();

    let cube: HashSet<Point> = input
        .split('\n')
        .filter_map(|v| into_point(v.split(',').filter_map(|x| x.parse::<i32>().ok())))
        .collect();

    let sum = cube.len() * 6 - prune_adjacent(&cube, &cube);

    println!("Part 1: {}", sum);

    let goal = get_cube_bounds(&cube).unwrap();

    let mut trapped_air: HashSet<Point> = HashSet::new();
    let mut free_air: HashSet<Point> = HashSet::new();

    for x in goal[0].x..goal[1].x {
        for y in goal[0].y..goal[1].y {
            for z in goal[0].z..goal[1].z {
                let pt = Point { x, y, z };

                let mut visited: HashSet<Point> = HashSet::new();
                if !cube.contains(&pt)
                    && !trapped_air.contains(&pt)
                    && !free_air.contains(&pt)
                    && dfs(
                        pt,
                        &goal,
                        &cube,
                        &mut visited,
                        &mut trapped_air,
                        &mut free_air,
                    ) != 1
                {
                    trapped_air.insert(pt);
                }
            }
        }
    }

    println!("Part 2: {:?}", sum - prune_adjacent(&cube, &trapped_air));
}
