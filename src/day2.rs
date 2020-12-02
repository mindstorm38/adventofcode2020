use std::str::FromStr;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("res/day2");

#[derive(Debug)]
struct Constraint {
    min: u16,
    max: u16,
    letter: char,
    password: String
}

impl Constraint {

    fn test_letters(&self) -> bool {
        (self.min..=self.max).contains(&(self.password.chars().filter(|c| *c == self.letter).count() as u16))
    }

    fn test_one_in_range(&self) -> bool {
        let a = self.password.chars().nth(self.min as usize - 1);
        let b = self.password.chars().nth(self.max as usize - 1);
        (a.is_some() && a.unwrap() == self.letter) ^ (b.is_some() && b.unwrap() == self.letter)
    }

}

fn main() {

    let constraints: Vec<Constraint> = INPUT.lines()
        .map(|l| {

            let parts: Vec<&str> = l.split(' ').collect();

            let bounds: Vec<u16> = parts[0].split('-')
                .map(u16::from_str)
                .map(Result::unwrap)
                .collect();

            Constraint {
                min: bounds[0],
                max: bounds[1],
                letter: parts[1].chars().nth(0).unwrap(),
                password: parts[2].to_string()
            }

        })
        .collect();

    println!("Count #1: {}", constraints.iter().filter(|c| c.test_letters()).count());
    println!("Count #2: {}", constraints.iter().filter(|c| c.test_one_in_range()).count());

}
