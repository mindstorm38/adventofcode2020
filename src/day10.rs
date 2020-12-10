use std::cmp::max;
use std::time::Instant;

const INPUT: &str = include_str!("res/day10");


fn count_jolts_differences(numbers: &Vec<u32>) -> (u32, u32) {

    let mut diff1 = 0u32;
    let mut diff3 = 0u32;

    for idx in 0..(numbers.len() - 1) {
        match numbers[idx + 1] - numbers[idx] {
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => {}
        }
    }

    (diff1, diff3)

}

fn total_combinations(numbers: &Vec<u32>) -> u64 {

    let mut accumulator: Vec<u64> = vec![1; numbers.len()];

    for (i, &num) in numbers.iter().enumerate().rev().skip(1) {

        let mut acc = 0u64;

        for j in (i + 1)..max(i + 4, numbers.len()) {
            let diff = numbers[j] - num;
            if diff <= 3 {
                acc += accumulator[j];
                if diff == 3 {
                    break;
                }
            } else {
                break;
            }
        }

        accumulator[i] = acc;

    }

    accumulator[0]

}

fn main() {

    let mut numbers: Vec<u32> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(*numbers.last().unwrap() + 3);

    let start = Instant::now();

    let (diff1, diff3) = count_jolts_differences(&numbers);
    println!("1-jolt diffs: {}", diff1);
    println!("3-jolts diffs: {}", diff3);
    println!("multiplied: {}", diff1 * diff3);

    let total = total_combinations(&numbers);
    println!("Total: {}", total);

    println!("Finished in {} ms", start.elapsed().as_secs_f64() * 1000f64);

}