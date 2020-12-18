use std::time::Instant;

const INPUT: &str = include_str!("res/day18");


#[repr(u8)]
#[derive(Debug)]
enum Op {
    Add, Mul
}


fn eval(line: &str, precedence: bool) -> u64 {

    let mut par_start = 0usize;
    let mut par_level = 0u16;

    let mut ops: Vec<Op> = Vec::new();
    let mut vals: Vec<u64> = Vec::new();

    for (i, ch) in line.chars().enumerate() {
        match ch {
            '*' if par_level == 0 => ops.push(Op::Mul),
            '+' if par_level == 0 => ops.push(Op::Add),
            '(' => {
                par_level += 1;
                if par_level == 1 {
                    par_start = i;
                }
            },
            ')' => {
                par_level -= 1;
                if par_level == 0 {
                    vals.push(eval(&line[(par_start + 1)..i], precedence));
                }
            },
            ' ' => {},
            _ if par_level == 0 => {
                if let Ok(num) = ch.to_string().parse::<u64>() {
                    vals.push(num);
                }
            },
            _ => {}
        }
    }

    if precedence {
        for (i, op) in ops.iter_mut().enumerate() {
            if let Op::Add = op {
                let v0 = vals[i];
                let v1 = vals[i + 1];
                vals[i] = 1;
                vals[i + 1] = v0 + v1;
                *op = Op::Mul;
            }
        }
    }

    let mut res = vals[0];

    for (i, op) in ops.into_iter().enumerate() {
        match op {
            Op::Mul => res *= vals[i + 1],
            Op::Add => res += vals[i + 1]
        }
    }

    res

}


fn main() {

    let global_start = Instant::now();

    let start = Instant::now();
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += eval(line, false);
    }
    println!("#1 Result: {} (in {}ms)", sum, start.elapsed().as_secs_f64() * 1000.0);

    let start = Instant::now();
    let mut sum = 0;
    for line in INPUT.lines() {
        sum += eval(line, true);
    }
    println!("#2 Result: {} (in {}ms)", sum, start.elapsed().as_secs_f64() * 1000.0);

    println!("All in {}ms", global_start.elapsed().as_secs_f64() * 1000.0);

}