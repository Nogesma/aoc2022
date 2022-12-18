extern crate core;

use memoize::memoize;
use ndarray::{Array, Array2};
use std::cmp::min;
use std::fs;

#[derive(Debug, Clone)]
struct Point {
    name: String,
    flow: u32,
    paths: Vec<String>,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .filter_map(|line| {
            let mut names = line
                .split(' ')
                .filter_map(|x| {
                    let xf = x.chars().filter(|c| c.is_alphabetic());
                    if xf.clone().all(|c| c.is_uppercase()) {
                        Some(xf.collect::<String>())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

            names.reverse();
            let name = names.pop()?;
            let (a, b) = (line.find('=')?, line.find(';')?);
            let flow = line[a + 1..b].parse().ok()?;

            Some(Point {
                name,
                flow,
                paths: names,
            })
        })
        .collect()
}

#[memoize]
fn dfs(
    cur: usize,
    rest: Vec<usize>,
    t: u32,
    dist: *const Array2<u32>,
    flows: *const Vec<u32>,
) -> (u32, Vec<usize>) {
    unsafe {
        let dist: &Array2<u32> = &*dist;
        let flows: &Vec<u32> = &*flows;

        rest.iter()
            .enumerate()
            .filter_map(|(idx, &v)| {
                if dist[[cur, v]] < t {
                    let mut rest = rest.clone();
                    rest.remove(idx);
                    let (a, b) = dfs(v, rest, t - dist[[cur, v]] - 1, dist, flows);
                    Some(((flows[v] * (t - dist[[cur, v]] - 1) + a), b))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default()
    }
}

fn dfs2(
    cur: usize,
    rest: Vec<usize>,
    t: u32,
    dist: *const Array2<u32>,
    flows: *const Vec<u32>,
) -> (u32, Vec<usize>) {
    unsafe {
        let dist: &Array2<u32> = &*dist;
        let flows: &Vec<u32> = &*flows;

        rest.iter()
            .enumerate()
            .map(|(idx, &v)| {
                if dist[[cur, v]] < t {
                    let mut rest = rest.clone();
                    rest.remove(idx);
                    let (a, b) = dfs2(v, rest, t - dist[[cur, v]] - 1, dist, flows);

                    let (c, d) = dfs(3, b, 26, dist, flows);
                    (flows[v] * (t - dist[[cur, v]] - 1) + a + c, d)
                } else {
                    (0, rest.clone())
                }
            })
            .max()
            .unwrap_or_default()
    }
}

pub fn main() {
    let input = &fs::read_to_string("day16/input").unwrap();

    let valves = parse(input);
    let mut dist = Array::from_elem((valves.len(), valves.len()), 99);

    for i in 0..valves.len() {
        for path in &valves[i].paths {
            let pos = valves.iter().position(|v| &v.name == path).unwrap();
            dist[[i, pos]] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                dist[[i, j]] = min(dist[[i, j]], dist[[i, k]] + dist[[k, j]]);
            }
        }
    }

    let start_idx = valves.iter().position(|v| &v.name == "AA").unwrap();
    let pressure_valves = valves
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| if v.flow > 0 { Some(idx) } else { None })
        .collect::<Vec<usize>>();

    let flows = valves.iter().map(|v| v.flow).collect::<Vec<u32>>();

    println!(
        "Part 1: {}",
        dfs(start_idx, pressure_valves.clone(), 30, &dist, &flows).0
    );

    println!(
        "Part 2: {}",
        dfs2(start_idx, pressure_valves, 26, &dist, &flows).0
    );
}
