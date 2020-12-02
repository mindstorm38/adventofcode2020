use std::str::FromStr;

const INPUT: &str = include_str!("res/day2");

type InputLine = (usize, usize, char, String);

fn parse(inp: impl Iterator<Item=String>) -> impl Iterator<Item=InputLine> {
    inp.map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();
        let bounds: Vec<&str> = parts[0].split('-').collect();
        (bounds[0].parse().unwrap(), bounds[1].parse().unwrap(), parts[1].chars().nth(0).unwrap(), parts[2].to_string())
    })
}

fn part1(inp: impl Iterator<Item=InputLine>) -> usize {
    inp.filter(|(min, max, car, pw)| {
        let char_count = pw.chars().filter(|&c| c == *car).count();
        (*min..=*max).contains(&char_count)
    }).count()
}

fn part2(inp: impl Iterator<Item=InputLine>) -> usize {
    inp.filter(|(min, max, car, pw)| {
        let a = pw.chars().nth(*min - 1).unwrap() == *car;
        let b = pw.chars().nth(*max - 1).unwrap() == *car;
        a ^ b
    }).count()
}

/*#[derive(Debug)]
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

    let t = INPUT.lines().map(String::from).collect::<Vec<String>>();

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

    let x;

    {
        let test = 1;
        x = &test;
    }



    println!("Count #1: {}", constraints.iter().filter(|c| c.test_letters()).count());
    println!("Count #2: {}", constraints.iter().filter(|c| c.test_one_in_range()).count());

}*/

fn main() {

    println!("Part1: {}", part1(parse(INPUT.lines().map(String::from))));
    println!("Part2: {}", part2(parse(INPUT.lines().map(String::from))));

}
