use libc::close;
use std::env;
use std::process::exit;
use std::time::Duration;

fn bench<F>(func: F) -> Duration
where
    F: Fn(),
{
    use std::time::Instant;

    println!("\nRunning {:?}:", std::any::type_name::<F>());

    let start = Instant::now();
    func();
    let elapsed = start.elapsed();

    println!("Execution time: {:?}", elapsed);

    elapsed
}

fn parse(func: &String) -> Duration {
    match func as &str {
        "day01" => bench(day01::main),
        "day02" => bench(day02::main),
        "day03" => bench(day03::main),
        "day04" => bench(day04::main),
        "day05" => bench(day05::main),
        "day06" => bench(day06::main),
        "day07" => bench(day07::main),
        "day08" => bench(day08::main),
        "day09" => bench(day09::main),
        "day10" => bench(day10::main),
        "day11" => bench(day11::main),
        "day12" => bench(day12::main),
        "day13" => bench(day13::main),
        "day14" => bench(day14::main),
        "day15" => bench(day15::main),
        "day16" => bench(day16::main),
        "day17" => bench(day17::main),
        // "day18" => bench(day18::main),
        // "day19" => bench(day19::main),
        // "day20" => bench(day20::main),
        // "day21" => bench(day21::main),
        // "day22" => bench(day22::main),
        // "day23" => bench(day23::main),
        // "day24" => bench(day24::main),
        arg => {
            eprintln!("Arg not recognized: {}", arg);
            exit(1);
        }
    }
}

fn main() {
    let days: Vec<String> = (1..=17).map(|x| format!("day{:<02}", x)).collect();
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
