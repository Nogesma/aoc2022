extern crate core;

use ndarray::iter::LanesMut;
use ndarray::{s, Array2, Ix1};
use std::fs;

fn calculate_trees_in_dir<'a>(
    mut slice: impl Iterator<Item = (usize, &'a mut (u32, u32))>,
) -> usize {
    let mut max = slice.next().unwrap().1 .0;
    let mut max_index: usize = 0;
    for (i, (treesize, _discovered)) in slice {
        if *treesize > max {
            max = *treesize;
            max_index = i;
            *_discovered = 1;
        }
        if *treesize == 9 {
            break;
        }
    }
    max_index
}

fn calculate_trees_for_slice(slices: LanesMut<(u32, u32), Ix1>) {
    for mut x in slices {
        calculate_trees_in_dir(x.indexed_iter_mut());
        calculate_trees_in_dir(x.iter_mut().rev().enumerate());
    }
}

fn p1(forest: &mut Array2<(u32, u32)>) -> usize {
    forest.row_mut(0).map_mut(|mut x| x.1 = 1);
    forest.row_mut(forest.nrows() - 1).map_mut(|mut x| x.1 = 1);
    forest.column_mut(0).map_mut(|mut x| x.1 = 1);
    forest
        .column_mut(forest.ncols() - 1)
        .map_mut(|mut x| x.1 = 1);

    calculate_trees_for_slice(forest.rows_mut());
    calculate_trees_for_slice(forest.columns_mut());
    forest.iter().fold(0, |acc, (_, x)| acc + *x as usize)
}

fn calculate_visible_trees<'a>(size: u32, slice: impl Iterator<Item = &'a (u32, u32)>) -> u32 {
    let mut visibility = 0;
    for (tree_size, _) in slice {
        visibility += 1;
        if *tree_size >= size {
            break;
        }
    }
    visibility
}

fn calculate_visibility(forest: &mut Array2<(u32, u32)>, i: usize, j: usize) {
    let size = forest[[i, j]].0;
    forest[[i, j]].1 *= calculate_visible_trees(size, forest.slice(s![..i, j]).iter().rev());
    forest[[i, j]].1 *= calculate_visible_trees(size, forest.slice(s![i + 1.., j]).iter());
    forest[[i, j]].1 *= calculate_visible_trees(size, forest.slice(s![i, ..j]).iter().rev());
    forest[[i, j]].1 *= calculate_visible_trees(size, forest.slice(s![i, j + 1..]).iter());
}

fn p2(forest: &mut Array2<(u32, u32)>) -> u32 {
    for i in 1..forest.nrows() - 1 {
        for j in 1..forest.ncols() - 1 {
            forest[[i, j]].1 = 1;
            calculate_visibility(forest, i, j);
        }
    }

    *forest.map(|(_, x)| *x).iter().max().unwrap()
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let width = input.find('\n').unwrap();
    let height = input.len() / width - 1;

    let forest: Array2<(u32, u32)> = Array2::from_shape_vec(
        (width, height),
        input
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| (x, 0))
            .collect::<Vec<(u32, u32)>>(),
    )
    .unwrap();

    println!("Part 1: {}", p1(&mut forest.clone()));
    println!("Part 2: {}", p2(&mut forest.clone()));
}
