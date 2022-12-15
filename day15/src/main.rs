extern crate core;

use std::collections::HashSet;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct Sensor {
    pos: [i32; 2],
    distance: i32,
}

fn parse(input: &str) -> (Vec<Sensor>, HashSet<[i32; 2]>) {
    let lines = input.split('\n').filter(|x| !x.is_empty());
    let mut sensors = vec![];
    let mut beacons: HashSet<[i32; 2]> = HashSet::new();

    for line in lines {
        let (a, b) = (line.find('=').unwrap(), line.find(',').unwrap());
        let x = line[a + 1..b].parse().unwrap();
        let line = &line[b..];

        let (a, b) = (line.find('=').unwrap(), line.find(':').unwrap());
        let y = line[a + 1..b].parse().unwrap();
        let line = &line[b..];

        let (a, b) = (line.find('=').unwrap(), line.find(',').unwrap());
        let bx = line[a + 1..b].parse().unwrap();
        let line = &line[b..];

        let a = line.find('=').unwrap();
        let by = line[a + 1..].parse().unwrap();

        beacons.insert([bx, by]);
        sensors.push(Sensor {
            pos: [x, y],
            distance: calculate_manhatan_distance(&[x, y], &[bx, by]),
        });
    }
    (sensors, beacons)
}

fn calculate_manhatan_distance(pos: &[i32; 2], beacon: &[i32; 2]) -> i32 {
    (pos[0] - beacon[0]).abs() + (pos[1] - beacon[1]).abs()
}

fn get_min_max_sensors(sensors: &Vec<Sensor>) -> (i32, i32) {
    let (mut min, mut max) = (0, 0);

    for sensor in sensors {
        if sensor.pos[0] - sensor.distance < min {
            min = sensor.pos[0] - sensor.distance;
        }
        if sensor.pos[0] + sensor.distance > max {
            max = sensor.pos[0] + sensor.distance;
        }
    }
    (min, max)
}

fn is_in_range(v: &i32, vec: &Vec<(i32, i32)>) -> bool {
    for (start, end) in vec {
        if v >= start && v <= end {
            return true;
        }
    }
    false
}

fn sum_of_ranges(ranges: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for (a, b) in ranges {
        sum += b - a;
    }
    sum
}

fn p1(sensors: &Vec<Sensor>, beacons: &HashSet<[i32; 2]>) -> i32 {
    let (min, max) = get_min_max_sensors(sensors);

    let mut vec: Vec<(i32, i32)> = vec![];
    let mut x = min;
    let mut a = x;
    'x: while x <= max {
        for sensor in sensors {
            let distance =
                sensor.distance - calculate_manhatan_distance(&sensor.pos, &[x, 2_000_000]) + 1;
            if distance > 0 {
                x += distance;
                continue 'x;
            }
        }
        if a != x {
            vec.push((a, x));
        }
        x += 1;
        a = x;
    }

    let mut res = 0;
    for [x, y] in beacons {
        if *y == 2_000_000 && is_in_range(x, &vec) {
            res += 1;
        }
    }
    sum_of_ranges(&vec) - res
}

fn gap_in_ranges(ranges: &mut Vec<Range<i32>>) -> i32 {
    ranges.sort_by(|a, b| a.end.cmp(&b.end));

    let mut stack: Vec<Range<i32>> = vec![ranges.pop().unwrap()];
    let mut last = stack.last_mut().unwrap();
    for i in (0..ranges.len()).rev() {
        if last.start < ranges[i].end {
            if last.start > ranges[i].start {
                last.start = ranges[i].start;
            }
        } else {
            stack.push(ranges[i].clone());
            last = stack.last_mut().unwrap();
        }
    }

    if stack.len() > 1 {
        if stack[1].end < stack[0].start {
            return stack[1].end;
        }
        if stack[1].start > 0 {
            return stack[1].start - 1;
        }
        if stack[0].end < LINE {
            return stack[0].end;
        }
    }

    0
}

const LINE: i32 = 4_000_000;

fn p2(sensors: &Vec<Sensor>) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut ranges: Vec<Range<i32>> = Vec::with_capacity(sensors.len());
    'y: while y < LINE {
        for sensor in sensors {
            let distance =
                sensor.distance - calculate_manhatan_distance(&sensor.pos, &[sensor.pos[0], y]);

            if distance >= 0 {
                ranges.push(sensor.pos[0] - distance..sensor.pos[0] + distance + 1);
            }
        }
        x = gap_in_ranges(&mut ranges);
        if x != 0 {
            break 'y;
        }
        ranges.clear();
        y += 1;
    }
    x as usize * LINE as usize + y as usize
}

fn main() {
    let input = &fs::read_to_string("input").unwrap();

    let (sensors, beacons) = parse(input);

    println!("Part 1: {}", p1(&sensors, &beacons));
    println!("Part 2: {}", p2(&sensors));
}
