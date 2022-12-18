extern crate core;

use std::fs;

const SCREEN_SIZE: usize = 40 * 6;

fn check_clock(clock: i32, x: i32) -> i32 {
    if (clock - 20) % 40 == 0 {
        x * clock
    } else {
        0
    }
}

fn draw_screen(screen: &mut [char; SCREEN_SIZE], clock: i32, x: i32) {
    let pos = clock % 40;
    if (x - 1..=x + 1).contains(&pos) {
        screen[clock as usize] = '#';
    }
}

fn part_1(input: &str) -> (i32, [char; SCREEN_SIZE]) {
    let mut clock = 0;
    let mut x = 1;
    let mut sum = 0;
    let mut screen: [char; SCREEN_SIZE] = ['.'; SCREEN_SIZE];

    let lines = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    for line in lines {
        if line == "noop" {
            draw_screen(&mut screen, clock, x);
            clock += 1;
            sum += check_clock(clock, x);
        } else {
            draw_screen(&mut screen, clock, x);
            clock += 1;
            sum += check_clock(clock, x);
            draw_screen(&mut screen, clock, x);
            clock += 1;
            sum += check_clock(clock, x);
            x += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        }
    }
    (sum, screen)
}

pub fn main() {
    let input = &fs::read_to_string("day10/input").unwrap();

    let (sum, screen) = part_1(input);
    println!("Part 1: {}\n", sum);

    print!("Part 2:");
    for (i, c) in screen.iter().enumerate() {
        if (i % 40) == 0 {
            println!();
        }
        print!("{}", c);
    }
    println!();
}
