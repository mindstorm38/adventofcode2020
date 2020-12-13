
#![allow(dead_code)]

use std::time::Instant;

const INPUT: &str = include_str!("res/day13");


fn parse(mut inp: impl Iterator<Item = String>) -> (u32, Vec<u32>) {
    let earliest_timestamp = inp.next().unwrap().parse().unwrap();
    let bus_ids = inp.next().unwrap().split(',')
        .map(|id| id.parse().unwrap_or(0))
        .collect();
    (earliest_timestamp, bus_ids)
}

fn debug_bus(bus_ids: &Vec<u32>) {

    print!("      | ");

    for &id in bus_ids {
        if id != 0 {
            print!(" Bus {:03} ", id);
        }
    }

    println!();

    for i in 0..bus_ids.len() {
        print!(" t+{:02} | ", i);
        for (j, &id) in bus_ids.iter().enumerate() {
            if id != 0 {
                let t = (i as i32 - j as i32).rem_euclid(id as i32);
                print!("{:03} ", t);
                if t == 0 {
                    print!(" x   ");
                } else {
                    print!("     ");
                }
            }
        }
        println!();
    }

}


/*fn gcd(mut a: u64, mut b: u64) -> u64 {

    if a < b {
        let t = b;
        b = a;
        a = t;
    }

    loop {
        let r = a % b;
        if r == 0 {
            return b;
        } else {
            a = b;
            b = r;
        }
    }

}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        panic!();
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_arr(mut arr: impl Iterator<Item = u64>) -> u64 {
    let start = arr.next().unwrap();
    arr.fold(start, |res, n| lcm(res, n))
}*/

// Source: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {

    let mut old_r = a as i64;
    let mut r = b as i64;

    let mut old_s = 1i64;
    let mut s = 0i64;

    let mut old_t = 0i64;
    let mut t = 1i64;

    while r != 0 {
        let quotient = old_r / r;
        let (or, os, ot) = (r, s, t);
        r = old_r - quotient * r;
        s = old_s - quotient * s;
        t = old_t - quotient * t;
        old_r = or;
        old_s = os;
        old_t = ot;
    }

    (old_r, old_s, old_t)

}


// Source: https://math.stackexchange.com/a/3864593
fn combine_phased_rotations(a_period: i64, a_phase: i64, b_period: i64, b_phase: i64) -> (i64, i64) {

    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_diff = a_phase - b_phase;

    let pd_mul = phase_diff / gcd;
    let pd_rem = phase_diff.rem_euclid(gcd);

    if pd_rem != 0 {
        panic!("Rotation reference points never sync.");
    }

    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase as i128 - s as i128 * pd_mul as i128 * a_period as i128).rem_euclid(combined_period as i128);

    (combined_period, combined_phase as i64)

}


fn main() {

    let start_global = Instant::now();

    let start = Instant::now();
    let (earliest_timestamp, bus_ids) = parse(INPUT.lines().map(String::from));
    println!("Parsed in {}ms", start.elapsed().as_secs_f64() * 1000.0);

    println!("Earliest timestamp: {}", earliest_timestamp);
    println!("Bus ids ({}): {:?}", bus_ids.len(), bus_ids);

    let start = Instant::now();

    let mut min_wait = u32::MAX;
    let mut min_bus_id = 0u32;

    for &id in &bus_ids {
        if id != 0 {

            let count = earliest_timestamp / id;
            let first = count * id;
            let second = (count + 1) * id;
            let diff = second - earliest_timestamp;

            if first == earliest_timestamp || diff == 0 {
                min_wait = 0;
                min_bus_id = id;
                break;
            } else if diff < min_wait {
                min_wait = diff;
                min_bus_id = id;
            }

        }
    }

    // debug_bus(&bus_ids);

    println!("Min wait: {}", min_wait);
    println!("Min bus id: {}", min_bus_id);
    println!("#1 Result: {}", min_wait * min_bus_id);
    println!("#1 In {}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = Instant::now();

    let mut period = 1;
    let mut phase = 0;

    for (i, &id) in bus_ids.iter().enumerate() {
        if id != 0 {
            let res = combine_phased_rotations(period, phase, id as i64, i as i64);
            period = res.0;
            phase = res.1;
        }
    }

    println!("#2 Period: {}", period);
    println!("#2 Phase: {}", phase);
    println!("#2 Result: {}", period - phase);
    println!("#2 In {}ms", start.elapsed().as_secs_f64() * 1000.0);

    println!("All in {}ms", start_global.elapsed().as_secs_f64() * 1000.0);

}