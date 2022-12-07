extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;

fn get_input() -> Enumerate<Lines<BufReader<File>>> {
    let path = Path::new("input");
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    reader.lines().enumerate()
}

fn build_dir(
    input: &mut Enumerate<Lines<BufReader<File>>>,
) -> Result<HashMap<String, usize>, &'static str> {
    let mut root = HashMap::new();

    root.insert(Path::new("/").to_str().unwrap().to_string(), 0);

    let mut current = Path::new("/");
    let (_, line) = input.next().unwrap();
    let mut line = line.unwrap();
    let mut tmp;

    while !line.is_empty() {
        if !line.starts_with('$') {
            return Err("Line doesn't start with $");
        }

        if line.contains("ls") {
            line = input.next().unwrap().1.unwrap();
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
                line = input.unwrap().1.unwrap();
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

            line = input.next().unwrap().1.unwrap();
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

fn get_size(filesystem: &HashMap<String, usize>, min: usize) -> usize {
    filesystem.iter().fold(usize::MAX, |accum, (path, _)| {
        let size = get_content_size(path.to_string(), filesystem);
        if size <= accum && size >= min {
            size
        } else {
            accum
        }
    })
}

fn main() {
    let mut input = get_input();

    let fs = build_dir(&mut input).unwrap();

    let needed_space = 30000000 - (70000000 - get_content_size("/".to_string(), &fs));

    println!("{}", get_size(&fs, needed_space));
}
