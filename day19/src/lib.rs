extern crate core;

use std::collections::HashSet;
use std::fs;
use std::mem::swap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Materials {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Debug)]
struct Blueprint {
    ore: Materials,
    clay: Materials,
    obsidian: Materials,
    geode: Materials,
    total: Materials,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    robots: Materials,
    mats: Materials,
    build: i32,
}

fn parse_blueprint(line: &str) -> Option<Blueprint> {
    let mut numbers = line.split(' ').filter_map(|x| x.parse().ok());

    let ore = Materials {
        ore: numbers.next()?,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    let clay = Materials {
        ore: numbers.next()?,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let obsidian = Materials {
        ore: numbers.next()?,
        clay: numbers.next()?,
        obsidian: 0,
        geode: 0,
    };

    let geode = Materials {
        ore: numbers.next()?,
        clay: 0,
        obsidian: numbers.next()?,
        geode: 0,
    };
    Some(Blueprint {
        ore,
        clay,
        obsidian,
        geode,
        total: Materials {
            ore: *[ore.ore, clay.ore, obsidian.ore, geode.ore]
                .iter()
                .max()
                .unwrap(),
            clay: obsidian.clay,
            obsidian: geode.obsidian,
            geode: 0,
        },
    })
}

fn mine_mats(robots: &Materials, mats: &mut Materials) {
    mats.ore += robots.ore;
    mats.clay += robots.clay;
    mats.obsidian += robots.obsidian;
    mats.geode += robots.geode;
}

fn can_build(cost: &Materials, mats: &Materials) -> bool {
    cost.ore <= mats.ore && cost.clay <= mats.clay && cost.obsidian <= mats.obsidian
}

fn apply_cost(cost: &Materials, mats: &mut Materials) {
    mats.ore -= cost.ore;
    mats.clay -= cost.clay;
    mats.obsidian -= cost.obsidian;
}

fn finish_building(robots: &mut Materials, to_build: i32) {
    match to_build {
        0 => robots.ore += 1,
        1 => robots.clay += 1,
        2 => robots.obsidian += 1,
        3 => robots.geode += 1,
        _ => {}
    }
}

fn bfs(blueprint: &Blueprint, mut state: State, t: i32) -> HashSet<State> {
    mine_mats(&state.robots, &mut state.mats);

    // Check if we have enough materials and robots to build max cost until last round
    let t = t - 1;
    if state.mats.ore > blueprint.total.ore
        && state.mats.ore + state.robots.ore * t > blueprint.total.ore * (t - 1)
    {
        state.mats.ore = blueprint.total.ore;
    }
    if state.mats.clay > blueprint.total.clay
        && state.mats.clay + state.robots.clay * t > blueprint.total.clay * (t - 1)
    {
        state.mats.clay = blueprint.total.clay;
    }
    if state.mats.clay > blueprint.total.clay
        && state.mats.obsidian + state.robots.obsidian * t > blueprint.total.obsidian * (t - 1)
    {
        state.mats.obsidian = blueprint.total.obsidian;
    }

    finish_building(&mut state.robots, state.build);

    let mut next = HashSet::with_capacity(4);

    if can_build(&blueprint.geode, &state.mats) {
        let mut state = state;
        apply_cost(&blueprint.geode, &mut state.mats);
        state.build = 3;
        next.insert(state);
    } else {
        if state.robots.obsidian < blueprint.total.obsidian
            && can_build(&blueprint.obsidian, &state.mats)
        {
            let mut state = state;
            apply_cost(&blueprint.obsidian, &mut state.mats);
            state.build = 2;
            next.insert(state);
        }

        if state.robots.clay < blueprint.total.clay && can_build(&blueprint.clay, &state.mats) {
            let mut state = state;
            apply_cost(&blueprint.clay, &mut state.mats);
            state.build = 1;
            next.insert(state);
        }

        if state.robots.ore < blueprint.total.ore && can_build(&blueprint.ore, &state.mats) {
            let mut state = state;
            apply_cost(&blueprint.ore, &mut state.mats);
            state.build = 0;
            next.insert(state);
        }

        next.insert(State {
            robots: state.robots,
            mats: state.mats,
            build: -1,
        });
    }

    next
}

#[inline]
fn maximum_possible_geodes(curr: i32, robots: i32, t: i32) -> i32 {
    curr + robots * t + (0..t).sum::<i32>()
}

fn get_current_max_geodes(states: &HashSet<State>, t: i32) -> i32 {
    states
        .iter()
        .map(|x| x.mats.geode + x.robots.geode * t)
        .max()
        .unwrap()
}

fn calculate_max_geodes(blueprint: &Blueprint, t: i32) -> usize {
    let start = State {
        robots: Materials {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        mats: Materials {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        build: -1,
    };

    let mut t = t;

    let mut curr = HashSet::new();

    let mut next = HashSet::new();
    curr.insert(start);

    while t != 0 {
        let max_geodes = get_current_max_geodes(&curr, t);

        let u: i64 = (curr.len()) as i64 - next.capacity() as i64;
        if u > 0 {
            next.reserve(u as usize);
        }
        for state in &curr {
            if maximum_possible_geodes(state.mats.geode, state.robots.geode, t) >= max_geodes {
                next.extend(bfs(blueprint, *state, t).iter());
            }
        }

        swap(&mut curr, &mut next);
        next.clear();
        t -= 1;
    }

    curr.iter().map(|x| x.mats.geode).max().unwrap() as usize
}

pub fn main() {
    let input = &fs::read_to_string("day19/input").unwrap();
    let blueprints = input.split('\n').filter_map(parse_blueprint);

    let p1 = blueprints
        .clone()
        .map(|x| calculate_max_geodes(&x, 24))
        .enumerate()
        .fold(0, |accum, (idx, v)| accum + (idx + 1) * v);

    println!("Part 1: {}", p1);

    let p2: usize = blueprints
        .take(3)
        .map(|x| calculate_max_geodes(&x, 32))
        .product();
    println!("Part 1: {}", p2);
}
