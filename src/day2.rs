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

fn main() {

    println!("Part1: {}", part1(parse(INPUT.lines().map(String::from))));
    println!("Part2: {}", part2(parse(INPUT.lines().map(String::from))));

}
