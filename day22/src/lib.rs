extern crate core;

use std::fs;

#[derive(Debug, Copy, Clone)]
#[repr(i8)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl TryFrom<i8> for Direction {
    type Error = &'static str;

    fn try_from(orig: i8) -> std::result::Result<Direction, &'static str> {
        match orig {
            0 => Ok(Direction::Right),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Up),
            _ => Err("Invalid value."),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Materials {
    Void,
    Wall,
    Open,
}

#[derive(Debug)]
struct Player {
    pos: (i32, i32),
    dir: Direction,
    p1: bool,
}

fn _print_map(map: &[Vec<Materials>], player: &Player) {
    println!("{:?}", player);
    for (x, row) in map.iter().enumerate() {
        for (y, mat) in row.iter().enumerate() {
            let c = if player.pos == (x as i32, y as i32) {
                match player.dir {
                    Direction::Up => '^',
                    Direction::Down => 'V',
                    Direction::Left => '<',
                    Direction::Right => '>',
                }
            } else {
                match mat {
                    Materials::Void => ' ',
                    Materials::Wall => '#',
                    Materials::Open => '.',
                }
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn get_wrapped_pos_p1(
    map: &[Vec<Materials>],
    (mut a, mut b): (i32, i32),
    dir: Direction,
) -> (usize, usize, Direction) {
    loop {
        let mut x;
        let mut y;
        match dir {
            Direction::Up | Direction::Down => {
                y = b as usize;
                x = if a < 0 { map.len() - 1 } else { a as usize };
                if x > map.len() - 1 {
                    x = 0;
                }
            }
            Direction::Left | Direction::Right => {
                x = a as usize;
                y = if b < 0 { map[x].len() - 1 } else { b as usize };
                if y > map[x].len() - 1 {
                    y = 0;
                }
            }
        }

        match map[x][y] {
            Materials::Void => {
                (a, b) = match dir {
                    Direction::Up => (x as i32 - 1, y as i32),
                    Direction::Down => (x as i32 + 1, y as i32),
                    Direction::Left => (x as i32, y as i32 - 1),
                    Direction::Right => (x as i32, y as i32 + 1),
                };
            }
            _ => {
                return (x, y, dir);
            }
        }
    }
}

fn get_wrapped_pos_p2(
    map: &[Vec<Materials>],
    (x, y): (i32, i32),
    dir: Direction,
) -> (usize, usize, Direction) {
    match dir {
        Direction::Up => {
            if x < 0 || map[x as usize][y as usize] == Materials::Void {
                if y / 50 == 0 {
                    (y as usize + 50, 50, Direction::Right)
                } else if y / 50 == 1 {
                    (y as usize + 100, 0, Direction::Right)
                } else {
                    (199, y as usize - 100, Direction::Up)
                }
            } else {
                (x as usize, y as usize, Direction::Up)
            }
        }
        Direction::Down => {
            if x as usize > map.len() - 1 || map[x as usize][y as usize] == Materials::Void {
                if y / 50 == 0 {
                    (0, 100 + y as usize, Direction::Down)
                } else if y / 50 == 1 {
                    (150 + (y as usize - 50), 49, Direction::Left)
                } else {
                    (50 + (y as usize - 100), 99, Direction::Left)
                }
            } else {
                (x as usize, y as usize, Direction::Down)
            }
        }
        Direction::Left => {
            if y < 0 || map[x as usize][y as usize] == Materials::Void {
                if x / 50 == 0 {
                    (49 - x as usize + 100, 0, Direction::Right)
                } else if x / 50 == 1 {
                    (100, (y + 1) as usize - 50, Direction::Down)
                } else if x / 50 == 2 {
                    (49 - (x as usize - 100), 50, Direction::Right)
                } else {
                    (0, x as usize - 100, Direction::Down)
                }
            } else {
                (x as usize, y as usize, Direction::Left)
            }
        }
        Direction::Right => {
            if y as usize > map[0].len() - 1 || map[x as usize][y as usize] == Materials::Void {
                if x / 50 == 0 {
                    (49 - x as usize + 100, 99, Direction::Left)
                } else if x / 50 == 1 {
                    (49, x as usize + 50, Direction::Up)
                } else if x / 50 == 2 {
                    (49 - (x as usize - 100), 149, Direction::Left)
                } else {
                    (149, x as usize - 100, Direction::Up)
                }
            } else {
                (x as usize, y as usize, Direction::Right)
            }
        }
    }
}

impl Player {
    fn rotate(&mut self, dir: bool) {
        self.dir = if dir {
            ((self.dir as i8 - 1).rem_euclid(4)).try_into().unwrap()
        } else {
            ((self.dir as i8 + 1).rem_euclid(4)).try_into().unwrap()
        }
    }

    fn m(&mut self, map: &[Vec<Materials>], v: i32) {
        let (mut x, mut y) = self.pos;
        let mut sd = self.dir;
        for _ in 0..v {
            let pos = match sd {
                Direction::Up => (x - 1, y),
                Direction::Down => (x + 1, y),
                Direction::Left => (x, y - 1),
                Direction::Right => (x, y + 1),
            };
            let (a, b, dir) = if self.p1 {
                get_wrapped_pos_p1(map, pos, sd)
            } else {
                get_wrapped_pos_p2(map, pos, sd)
            };
            match map[a][b] {
                Materials::Wall => break,
                Materials::Open => {
                    (x, y) = (a as i32, b as i32);
                    sd = dir;
                }
                Materials::Void => panic!(),
            }
        }
        self.pos = (x, y);
        self.dir = sd;
    }
}

#[derive(Debug)]
enum Path {
    Move { v: i32 },
    Rotation { dir: bool },
}

fn apply_move(map: &[Vec<Materials>], player: &mut Player, v: &Path) {
    match *v {
        Path::Rotation { dir } => player.rotate(dir),
        Path::Move { v } => player.m(map, v),
    }
}

pub fn main() {
    let input = &fs::read_to_string("day22/input").unwrap();

    let lines = input.split('\n');
    let path_str = lines.clone().rev().skip(1).take(1).collect::<String>();

    let mut start = 0;
    let mut path: Vec<Path> = Vec::new();

    for (end, char) in path_str.chars().enumerate() {
        if char == 'R' || char == 'L' {
            if end != start {
                let v = path_str[start..end].parse().unwrap();
                path.push(Path::Move { v });
                if char == 'R' {
                    path.push(Path::Rotation { dir: false });
                } else if char == 'L' {
                    path.push(Path::Rotation { dir: true });
                }
            }
            start = end + 1;
        }
    }
    let char = path_str.chars().last().unwrap();
    let end = path_str.len();
    if path_str.len() - 1 != start {
        let v = path_str[start..end].parse().unwrap();
        path.push(Path::Move { v });
        if char == 'R' {
            path.push(Path::Rotation { dir: false });
        } else if char == 'L' {
            path.push(Path::Rotation { dir: true });
        }
    }

    let mut map = lines
        .filter_map(|x| {
            let row = x
                .chars()
                .filter_map(|c| match c {
                    ' ' => Some(Materials::Void),
                    '.' => Some(Materials::Open),
                    '#' => Some(Materials::Wall),
                    _ => None,
                })
                .collect::<Vec<Materials>>();
            if row.len() != x.len() || row.is_empty() {
                None
            } else {
                Some(row)
            }
        })
        .collect::<Vec<Vec<Materials>>>();

    let first_open = map[0]
        .iter()
        .position(|x| matches!(x, Materials::Open))
        .unwrap();

    let mut player = Player {
        pos: (0, first_open as i32),
        dir: Direction::Right,
        p1: true,
    };

    let width = map
        .iter()
        .max_by(|&a, &b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    for row in map.iter_mut() {
        if row.len() < width {
            row.append(&mut (row.len()..width).map(|_| Materials::Void).collect());
        }
    }

    path.iter().for_each(|v| apply_move(&map, &mut player, v));
    println!(
        "Part 1: {:?}",
        (player.pos.0 + 1) * 1000 + (player.pos.1 + 1) * 4 + player.dir as i32
    );

    let mut player = Player {
        pos: (0, first_open as i32),
        dir: Direction::Right,
        p1: false,
    };

    path.iter().for_each(|v| apply_move(&map, &mut player, v));
    println!(
        "Part 2: {:?}",
        (player.pos.0 + 1) * 1000 + (player.pos.1 + 1) * 4 + player.dir as i32
    );
}
