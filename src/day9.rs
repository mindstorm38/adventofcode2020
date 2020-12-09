use std::time::Instant;

const INPUT: &str = include_str!("res/day9");

fn find_weakness(inp: &Vec<u64>, size: usize) -> Option<u64> {

    let mut queue: Vec<u64> = Vec::with_capacity(size);
    let mut found: bool;

    for &val in inp {
        if queue.len() >= size {
            found = false;
            'a: for (i, &a) in queue.iter().enumerate() {
                for &b in queue.iter().skip(i + 1) {
                    if a + b == val {
                        found = true;
                        break'a;
                    }
                }
            }
            if !found {
                return Some(val);
            }
            queue.remove(0);
        }
        queue.push(val);
    }

    None

}

fn find_contiguous_sum(inp: &Vec<u64>, expected: u64) -> &[u64] {

    let mut from = 0usize;
    let mut to = 0usize;
    let mut sum = 0u64;

    loop {

        while sum < expected {
            sum += inp[to];
            to += 1;
        }

        if sum == expected {
            break;
        }

        while sum > expected {
            sum -= inp[from];
            from += 1;
        }

        if sum == expected {
            break;
        }

    }

    &inp[from..to]

}

fn main() {

    let numbers: Vec<u64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    let start_global = Instant::now();

    let start = Instant::now();
    let weakness = find_weakness(&numbers, 25).unwrap();
    println!("Found weakness: {} (in {}ms)", weakness, start.elapsed().as_secs_f64() * 1000f64);

    let start = Instant::now();
    let set = find_contiguous_sum(&numbers, weakness);
    let min = *set.iter().min().unwrap();
    let max = *set.iter().max().unwrap();
    println!("Found real weakness: {} + {} = {} (in {}ms)", min, max, min + max, start.elapsed().as_secs_f64() * 1000f64);
    println!("Total in {}ms", start_global.elapsed().as_secs_f64() * 1000f64);

}