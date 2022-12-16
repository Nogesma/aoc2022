use libc::close;
use std::env;
use std::process::exit;
use std::time::Duration;

use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use day09::day09;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
// use day16::day16;
// use day17::day17;
// use day18::day18;
// use day19::day19;
// use day20::day20;
// use day21::day21;
// use day22::day22;
// use day23::day23;
// use day24::day24;

fn bench(func: fn()) -> Duration {
    use std::time::Instant;

    // println!("\nRunning {:?}:", func);

    let start = Instant::now();
    func();
    let elapsed = start.elapsed();

    println!("Execution time: {:?}", elapsed);

    elapsed
}

fn parse(func: &String) -> Duration {
    match func as &str {
        "day01" => bench(day01),
        "day02" => bench(day02),
        "day03" => bench(day03),
        "day04" => bench(day04),
        "day05" => bench(day05),
        "day06" => bench(day06),
        "day07" => bench(day07),
        "day08" => bench(day08),
        "day09" => bench(day09),
        "day10" => bench(day10),
        "day11" => bench(day11),
        "day12" => bench(day12),
        "day13" => bench(day13),
        "day14" => bench(day14),
        "day15" => bench(day15),
        // "day16" => bench(day16),
        // "day17" => bench(day17),
        // "day18" => bench(day18),
        // "day19" => bench(day19),
        // "day20" => bench(day20),
        // "day21" => bench(day21),
        // "day22" => bench(day22),
        // "day23" => bench(day23),
        // "day24" => bench(day24),
        arg => {
            println!("Arg not recognized: {}", arg);
            exit(1);
        }
    }
}

fn main() {
    let days: Vec<String> = (1..=15).map(|x| format!("day{:<02}", x)).collect();
    let args: Vec<String> = env::args().skip(1).collect();

    args.iter().for_each(|v| match v.as_str() {
        "all" => {
            let durations = days.iter().map(parse);
            eprintln!("\nTotal: {:?}", durations.sum::<Duration>());
        }
        "bench" => unsafe {
            close(1);

            let durations = (0..10).flat_map(|_| days.iter().map(parse));

            eprintln!("Average: {:?}", durations.sum::<Duration>() / 10);
        },
        _ => {
            parse(v);
        }
    })
}
