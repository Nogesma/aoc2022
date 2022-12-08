extern crate core;

use ndarray::{s, Array2};
use std::fs;

fn calculate_visible_trees(size: u32, slice: impl Iterator<Item = (u32, u32)>) -> u32 {
    let mut visibility = 0;
    for (tree_size, _) in slice {
        visibility += 1;
        if tree_size >= size {
            break;
        }
    }
    visibility
}

fn calculate_visibility(forest: &mut Array2<(u32, u32)>, i: usize, j: usize) {
    forest[[i, j]].1 *= calculate_visible_trees(
        forest[[i, j]].0,
        forest.slice(s![..i, j]).iter().rev().cloned(),
    );
    forest[[i, j]].1 *= calculate_visible_trees(
        forest[[i, j]].0,
        forest.slice(s![i + 1.., j]).iter().cloned(),
    );
    forest[[i, j]].1 *= calculate_visible_trees(
        forest[[i, j]].0,
        forest.slice(s![i, ..j]).iter().rev().cloned(),
    );
    forest[[i, j]].1 *= calculate_visible_trees(
        forest[[i, j]].0,
        forest.slice(s![i, j + 1..]).iter().cloned(),
    );
}

fn get_visible_trees(forest: &mut Array2<(u32, u32)>) -> u32 {
    for i in 1..forest.nrows() - 1 {
        for j in 1..forest.ncols() - 1 {
            calculate_visibility(forest, i, j);
        }
    }

    *forest.map(|(_, x)| *x).iter().max().unwrap()
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let width = input.find('\n').unwrap();
    let height = input.len() / width - 1;
    let mut forest: Array2<(u32, u32)> = Array2::from_shape_vec(
        (width, height),
        input
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| (x, 1))
            .collect::<Vec<(u32, u32)>>(),
    )
    .unwrap();

    println!("{}", get_visible_trees(&mut forest));
}
