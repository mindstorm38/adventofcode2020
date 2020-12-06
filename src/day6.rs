const INPUT: &str = include_str!("res/day6");

#[inline]
fn get_char_mask(ch: char) -> u32 {
    1 << (ch as u32 - 'a' as u32)
}

fn count_anyone_questions(inp: impl Iterator<Item=String>) -> usize {

    let mut answered = 0u32;
    let mut count = 0usize;

    for line in inp {
        if line.is_empty() {
            answered = 0;
        } else {
            for ch in line.chars() {
                let mask = get_char_mask(ch);
                if answered & mask == 0 {
                    answered |= mask;
                    count += 1;
                }
            }
        }
    }

    count

}

fn count_everyone_questions(inp: impl Iterator<Item=String>) -> usize {

    let mut answered: u32;
    let mut answers_acc = 0u32;

    let mut first = true;
    let mut count = 0usize;

    for line in inp {
        if line.is_empty() {
            first = true;
            count += answers_acc.count_ones() as usize;
        } else {
            answered = 0;
            for ch in line.chars() {
                answered |= get_char_mask(ch);
            }
            if first {
                answers_acc = answered;
                first = false;
            } else {
                answers_acc &= answered;
            }
        }
    }

    if !first {
        count += answers_acc.count_ones() as usize;
    }

    count

}

fn main() {

    println!("Count anyone: {}", count_anyone_questions(INPUT.lines().map(String::from)));
    println!("Count everyone: {}", count_everyone_questions(INPUT.lines().map(String::from)));

}