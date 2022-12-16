extern crate core;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn build_dir(input: &str) -> Result<HashMap<String, usize>, &'static str> {
    let mut root = HashMap::new();

    root.insert(Path::new("/").to_str().unwrap().to_string(), 0);

    let mut input = input.split('\n');
    let mut current = Path::new("/");
    let mut line = input.next().unwrap();
    let mut tmp;

    while !line.is_empty() {
        if !line.starts_with('$') {
            return Err("Line doesn't start with $");
        }

        if line.contains("ls") {
            line = input.next().unwrap();
            while !line.starts_with('$') && !line.is_empty() {
                let attributes: [&str; 2] = line
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                if !line.starts_with("dir") {
                    if let Some(x) = root.get_mut(current.to_str().unwrap()) {
                        *x += attributes[0].parse::<usize>().unwrap();
                    } else {
                        return Err("Current path not found in map");
                    }
                };

                let input = input.next();
                if input.is_none() {
                    return Ok(root);
                }
                line = input.unwrap();
            }
        } else if line.contains("cd") {
            let attributes: [&str; 3] = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            if attributes[2].eq("..") {
                current = current.parent().unwrap_or_else(|| Path::new("/"));
            } else {
                tmp = current.join(attributes[2]);
                current = tmp.as_path();

                root.insert(current.to_str().unwrap().to_string(), 0);
            }

            line = input.next().unwrap();
        }
    }
    Ok(root)
}

fn get_content_size(current: String, filesystem: &HashMap<String, usize>) -> usize {
    filesystem.iter().fold(0, |accum, (path, size)| {
        if Path::new(path).starts_with(current.clone()) {
            accum + size
        } else {
            accum
        }
    })
}

fn p1(filesystem: &HashMap<String, usize>) -> usize {
    filesystem.iter().fold(0, |accum, (path, _)| {
        let size = get_content_size(path.to_string(), filesystem);
        if size <= 100_000 {
            accum + size
        } else {
            accum
        }
    })
}

fn p2(filesystem: &HashMap<String, usize>, min: usize) -> usize {
    filesystem.iter().fold(usize::MAX, |accum, (path, _)| {
        let size = get_content_size(path.to_string(), filesystem);
        if size <= accum && size >= min {
            size
        } else {
            accum
        }
    })
}

pub fn day07() {
    let input = &fs::read_to_string("day07/input").unwrap();

    let fs = build_dir(input).unwrap();

    println!("Part 1: {}", p1(&fs));

    let needed_space = 30000000 - (70000000 - get_content_size("/".to_string(), &fs));
    println!("Part 2: {}", p2(&fs, needed_space));
}
