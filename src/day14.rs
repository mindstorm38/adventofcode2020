use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("res/day14");
const BITS: usize = 36;

#[derive(Clone)]
enum Instruction {
    Mask(Vec<u8>),
    Set(u64, u64)
}

fn parse(inp: impl Iterator<Item = String>) -> Vec<Instruction> {
    inp.map(|line| {
        use Instruction::*;
        if &line[..7] == "mask = " {
            let mask = &line[7..];
            Mask(mask.chars().take(BITS).map(|ch| match ch {
                '0' => 0,
                '1' => 1,
                _ => 2
            }).collect())
        } else {
            let equal_idx = line.find('=').unwrap();
            let address = (&line[4..(equal_idx - 2)]).parse().unwrap();
            let val = (&line[(equal_idx + 2)..]).parse().unwrap();
            Set(address, val)
        }
    }).collect()
}

fn insert(memory: &mut HashMap<u64, u64>, sum: &mut u64, addr: u64, val: u64) {
    *sum += val;
    if let Some(old_val) = memory.insert(addr, val) {
        *sum -= old_val;
    }
}

fn insert_floating(memory: &mut HashMap<u64, u64>, sum: &mut u64, mask: &Vec<u8>, mut addr: u64, val: u64) {

    let mut floating_bits: Vec<usize> = Vec::new();

    for (i, &m) in mask.iter().enumerate() {
        let i = BITS - 1 - i;
        match m {
            1 => addr |= 1 << i,
            2 => floating_bits.push(i),
            _ => {}
        }
    }

    /*println!("With mask {}", mask.iter().map(|&m| m.to_string()).collect::<String>());
    println!("     Addr {:036b}", addr);
    println!("    FBits {:?}", floating_bits);*/

    insert_floating_worker(memory, sum, addr, &floating_bits[..], val);

}

fn insert_floating_worker(memory: &mut HashMap<u64, u64>, sum: &mut u64, addr: u64, floating_bits: &[usize], val: u64) {
    if !floating_bits.is_empty() {
        let i = floating_bits[0];
        let new_floating_bits = &floating_bits[1..];
        insert_floating_worker(memory, sum, addr & !(1 << i), new_floating_bits, val);
        insert_floating_worker(memory, sum, addr | (1 << i), new_floating_bits, val);
    } else {
        // println!("Insert {:036b} => {}", addr, val);
        insert(memory, sum, addr, val);
    }
}

fn exec(program: &Vec<Instruction>, mask_addr: bool) -> u64 {

    let mut current_mask: Vec<u8> = Vec::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut sum = 0u64;

    for inst in program {
        match inst {
            Instruction::Mask(mask) => {
                current_mask.clear();
                current_mask.extend_from_slice(&mask[..]);
            }
            &Instruction::Set(addr, mut val) => {

                if mask_addr {
                    insert_floating(&mut memory, &mut sum, &current_mask, addr, val);
                } else {

                    for (i, &m) in current_mask.iter().enumerate() {
                        let i = BITS - 1 - i;
                        match m {
                            0 => val &= !(1 << i),
                            1 => val |= 1 << i,
                            _ => {}
                        }
                    }

                    insert(&mut memory, &mut sum, addr, val);

                }

            }
        }
    }

    sum

}

fn main() {

    let start_global = Instant::now();

    let start = Instant::now();
    let program = parse(INPUT.lines().map(String::from));
    println!("Parsed in {}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = Instant::now();
    let sum1 = exec(&program, false);
    println!("Memory sum 1: {} (in {}ms)", sum1, start.elapsed().as_secs_f64() * 1000.0);

    let start = Instant::now();
    let sum2 = exec(&program, true);
    println!("Memory sum 2: {} (in {}ms)", sum2, start.elapsed().as_secs_f64() * 1000.0);

    println!("Total in {}ms", start_global.elapsed().as_secs_f64() * 1000.0);

}