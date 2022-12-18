extern crate core;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::iter::Peekable;

fn move_left(map: &[u8], rock: &mut [u8; 4], pos: usize) {
    for i in 0..rock.len() {
        if (rock[i] & 0b1000000) != 0 {
            return;
        }

        if map[pos + i] & rock[i] << 1 != 0 {
            return;
        }
    }
    for i in 0..rock.len() {
        rock[i] <<= 1;
    }
}

fn move_right(map: &[u8], rock: &mut [u8; 4], pos: usize) {
    for i in 0..rock.len() {
        if (rock[i] & 0b0000001) != 0 {
            return;
        }

        if map[pos + i] & rock[i] >> 1 != 0 {
            return;
        }
    }
    for i in 0..rock.len() {
        rock[i] >>= 1;
    }
}

fn can_drop(map: &[u8], rock: &[u8; 4], pos: usize) -> bool {
    for i in 0..rock.len() {
        if map[pos - 1 + i] & rock[i] != 0 {
            return false;
        }
    }
    true
}

fn stop(map: &mut [u8], rock: &[u8; 4], pos: usize) -> usize {
    let mut ref_line = 0;
    for i in 0..rock.len() {
        map[pos + i] |= rock[i];
        if map[pos + i] == map[1] {
            ref_line = pos + i
        }
    }
    ref_line
}

fn _print_map(map: &[u8]) {
    for row in map.iter().rev() {
        println!("{:07b}", row);
    }
    println!();
    println!();
}

pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: Iterator> PeekableIterator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        Peekable::peek(self)
    }
}

static ROCKS: [[u8; 4]; 5] = [
    [0b0011110, 0b0000000, 0b0000000, 0b0000000],
    [0b0001000, 0b0011100, 0b0001000, 0b0000000],
    [0b0011100, 0b0000100, 0b0000100, 0b0000000],
    [0b0010000, 0b0010000, 0b0010000, 0b0010000],
    [0b0011000, 0b0011000, 0b0000000, 0b0000000],
];

fn tetris(
    cycles: usize,
    map: &mut Vec<u8>,
    mut shapes: impl PeekableIterator<Item = usize>,
    mut jets: impl PeekableIterator<Item = (usize, char)>,
) -> usize {
    let mut state: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut ref_line = 0;
    for idx in 0..cycles {
        let highest_rock = map.len() - 1 - map.iter().rev().position(|&x| x > 0).unwrap();
        let mut s = DefaultHasher::new();
        (shapes.peek(), jets.peek(), highest_rock - ref_line + 1).hash(&mut s);
        let hash = s.finish();

        let x = state.get_mut(&hash);
        if let Some((i, cycle_start_height)) = x {
            let cycle_start_height = *cycle_start_height;

            let cycle_size = idx - *i;
            let remaining_cycles = cycles - idx;
            let cycle_height =
                (highest_rock - cycle_start_height) * (remaining_cycles / cycle_size + 1);
            let remaining_height =
                tetris(remaining_cycles % cycle_size, map, shapes, jets) - highest_rock;
            return cycle_height + remaining_height + cycle_start_height;
        } else {
            state.insert(hash, (idx, highest_rock));
        }

        let shape_idx = shapes.next().unwrap();

        for _ in map.len()..=highest_rock + 7 {
            map.push(0b0000000);
        }

        let mut rock = ROCKS[shape_idx];
        let mut pos = highest_rock + 4;

        loop {
            let dir = jets.next().unwrap().1;
            match dir {
                '<' => move_left(map, &mut rock, pos),
                '>' => move_right(map, &mut rock, pos),
                _ => panic!(),
            }
            if can_drop(map, &rock, pos) {
                pos -= 1;
            } else {
                let tmp = stop(map, &rock, pos);
                if tmp != 0 {
                    ref_line = tmp;
                }
                break;
            }
        }
    }
    map.len() - 1 - map.iter().rev().position(|&x| x > 0).unwrap()
}

pub fn main() {
    let input = &fs::read_to_string("day17/input").unwrap();

    let jets = input
        .as_str()
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .cycle()
        .peekable();

    let mut map: Vec<u8> = vec![0b1111111];

    println!(
        "Part 1: {}",
        tetris(
            2022,
            &mut map.clone(),
            (0..5).into_iter().cycle().peekable(),
            jets.clone()
        )
    );

    println!(
        "Part 2: {}",
        tetris(
            1000000000000,
            &mut map,
            (0..5).into_iter().cycle().peekable(),
            jets
        )
    );
}
