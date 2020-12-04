use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;
use regex::Regex;
use std::fmt::Display;

const INPUT: &str = include_str!("res/day4");
const MANDATORY_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];


fn filter_passports(inp: impl Iterator<Item=String>) -> impl Iterator<Item=String> {
    inp.map(|raw_passport| raw_passport.split("\n").filter(|&line| !line.is_empty()).collect::<Vec<&str>>().join(" "))
}

fn count_valid_passports(inp: impl Iterator<Item=String>) -> usize {
    inp.filter(|passport| {
        passport.split(' ').filter(|&kv| MANDATORY_FIELDS.contains(&&kv[..kv.find(':').unwrap()])).count() == MANDATORY_FIELDS.len()
    }).count()
}

fn constraint_range<T: 'static + PartialOrd<T> + FromStr + Display>(range: RangeInclusive<T>) -> Box<dyn Fn(&str) -> bool> {
    Box::new(move |val| match val.parse::<T>() {
        Ok(i) => range.contains(&i),
        _ => false
    })
}

fn get_constraint(key: &str) -> Option<Box<dyn Fn(&str) -> bool>> {
    match key {
        "byr" => Some(constraint_range(1920..=2002)),
        "iyr" => Some(constraint_range(2010..=2020)),
        "eyr" => Some(constraint_range(2010..=2030)),
        "hgt" => Some(Box::new(|val| {
            let limit = val.len() - 2;
            match &val[..limit].parse::<i32>() {
                Ok(i) => match &val[limit..] {
                    "cm" => (150..=193).contains(i),
                    "in" => (59..=76).contains(i),
                    _ => false
                },
                _ => false
            }
        })),
        "hcl" => Some(Box::new(|val| Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(val))),
        "ecl" => Some(Box::new(|val| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(val))),
        "pid" => Some(Box::new(|val| Regex::new(r"^[0-9]{9}$").unwrap().is_match(val))),
        _ => None
    }
}

fn count_valid_passports_2(inp: impl Iterator<Item=String>) -> usize {
    inp.filter(|passport| {
        passport.split(' ').filter(|&kv| {
            let sep = kv.find(':').unwrap();
            match get_constraint(&kv[..sep]) {
                Some(val) => val.deref()(&kv[(sep + 1)..]),
                None => false
            }
        }).count() == MANDATORY_FIELDS.len()
    }).count()
}


fn main() {

    let count = count_valid_passports(filter_passports(INPUT.split("\n\n").map(String::from)));
    println!("Part 1: {}", count);

    let count = count_valid_passports_2(filter_passports(INPUT.split("\n\n").map(String::from)));
    println!("Part 2: {}", count);

}