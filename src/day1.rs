use std::str::FromStr;

const INPUT: &str = include_str!("res/day1");

fn main() {

    let values: Vec<u32> = INPUT.lines().map(|l| u32::from_str(l).unwrap()).collect();

    println!("Vec size: {}", values.len());

    let mut op_count = 0usize;

    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i) {
            op_count += 1;
            if x + y == 2020 {
                println!("{} x {} = {}", x, y, x * y);
            }
        }
    }

    println!("Op Count: {}", op_count);

    op_count = 0;

    for (i, x) in values.iter().enumerate() {
        for (j, y) in values.iter().enumerate().skip(i) {
            for z in values.iter().skip(j) {
                op_count += 1;
                if x + y + z == 2020 {
                    println!("{} x {} x {} = {}", x, y, z, x * y * z);
                }
            }
        }
    }

    println!("Op Count: {}", op_count);

}