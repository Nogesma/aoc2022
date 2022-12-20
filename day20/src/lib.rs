extern crate core;

use std::fs;

fn wrap_index(v: &Vec<usize>, idx: i64) -> usize {
    idx.rem_euclid(v.len() as i64) as usize
}

fn find(vec: &[usize], v: &usize) -> Option<usize> {
    vec.iter().position(|x| x == v)
}

fn mix(indexes: &mut Vec<usize>, values: &[i64]) {
    for x in values.iter().enumerate() {
        let (i, v) = x;
        let current_index = find(indexes, &i).unwrap();

        indexes.remove(current_index);
        let new_idx = wrap_index(indexes, current_index as i64 + v);
        indexes.insert(new_idx, i);
    }
}

fn get_result(indexes: &Vec<usize>, values: &[i64]) -> i64 {
    let zero_idx = values.iter().position(|&v| v == 0).unwrap();
    let idx = find(indexes, &zero_idx).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|x| values[indexes[wrap_index(indexes, idx as i64 + x)]])
        .sum()
}

pub fn main() {
    let input = &fs::read_to_string("day20/input").unwrap();
    let values: Vec<i64> = input.split('\n').filter_map(|x| x.parse().ok()).collect();
    let indexes: Vec<usize> = (0..values.len()).collect();

    let mut midx = indexes.clone();
    mix(&mut midx, &values);

    println!("Part 1: {}", get_result(&midx, &values));

    let values: Vec<i64> = values.iter().map(|x| x * 811589153).collect();

    let mut midx = indexes;
    (0..10).for_each(|_| mix(&mut midx, &values));
    println!("Part 2: {}", get_result(&midx, &values));
}
