extern crate core;

use ndarray::iter::LanesMut;
use ndarray::{Array2, Ix1};
use std::fs;

fn calculate_trees_for_slice<'a>(
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

fn calculate_trees(slices: LanesMut<(u32, u32), Ix1>) {
    for mut x in slices {
        calculate_trees_for_slice(x.indexed_iter_mut());
        calculate_trees_for_slice(x.iter_mut().rev().enumerate());
    }
}

fn get_visible_trees(forest: &mut Array2<(u32, u32)>) -> usize {
    forest.row_mut(0).map_mut(|mut x| x.1 = 1);
    forest.row_mut(forest.nrows() - 1).map_mut(|mut x| x.1 = 1);
    forest.column_mut(0).map_mut(|mut x| x.1 = 1);
    forest
        .column_mut(forest.ncols() - 1)
        .map_mut(|mut x| x.1 = 1);

    calculate_trees(forest.rows_mut());
    calculate_trees(forest.columns_mut());

    forest.iter().fold(0, |acc, (_, x)| acc + *x as usize)
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
            .map(|x| (x, 0))
            .collect::<Vec<(u32, u32)>>(),
    )
    .unwrap();

    println!("{}", get_visible_trees(&mut forest));
}
