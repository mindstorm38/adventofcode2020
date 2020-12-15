use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("res/day15");

#[derive(Debug)]
struct NumberStat {
    index: usize,
    first: bool
}

fn run_sequence(numbers: &Vec<u32>, times: usize) -> (f64, f64, u32) {

    let now = Instant::now();
    let loops = times - numbers.len();

    let mut last_value = *numbers.last().unwrap();
    let mut cache: HashMap<u32, usize> = HashMap::new();

    for (i, &n) in numbers.iter().enumerate() {
        cache.insert(n, i);
    }

    for i in numbers.len()..times {

        let value = if let Some(&j) = cache.get(&last_value) {
            (i - 1 - j) as u32
        } else {
            0
        };

        cache.insert(last_value, i - 1);
        last_value = value;

    }

    let total_time = now.elapsed().as_secs_f64();
    let average_loop = total_time / loops as f64;

    (total_time, average_loop, last_value)

}

fn main() {

    const PART1_TIMES: usize = 2020;
    const PART2_TIMES: usize = 30000000;

    let numbers: Vec<u32> = INPUT.split(",").map(|n| n.parse().unwrap()).collect();

    let (total_time, average_loop, last_num) = run_sequence(&numbers, PART1_TIMES);
    println!("#1 Last number: {} (in {:.3}ms, average loop at {:.4}ms)", last_num, total_time * 1000.0, average_loop * 1000.0);
    println!("#2 This will take {:.0}s for part 2 ...", average_loop * PART2_TIMES as f64);
    let (total_time, average_loop, last_num) = run_sequence(&numbers, PART2_TIMES);
    println!("#2 Last number: {} (in {:.3}s, average loop at {:.4}ms)", last_num, total_time, average_loop * 1000.0);

}