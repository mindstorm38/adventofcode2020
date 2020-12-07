use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("res/day7");

type BagRules = HashMap<String, HashMap<String, u32>>;

fn parse_rules(inp: impl Iterator<Item=String>) -> BagRules {

    let mut rules = HashMap::new();

    for line in inp {

        let (color, content_raw) = line.split_at(line.find(" bags contain ").unwrap());
        let content_raw = &content_raw[" bags contain ".len()..];

        let mut content: HashMap<String, u32> = HashMap::new();

        if content_raw != "no other bags." {
            for bag_in in content_raw.split(", ") {
                let (count_raw, in_color) = bag_in.split_at(bag_in.find(' ').unwrap());
                let in_color = &in_color[1..in_color.rfind(" bag").unwrap()];
                content.insert(in_color.to_string(), count_raw.parse().unwrap());
            }
        }

        rules.insert(color.to_string(), content);

    }

    rules

}

fn count_bags_containing(rules: &BagRules, search_color: String) -> usize {

    let mut directly_containing: HashSet<String> = HashSet::new();
    directly_containing.insert(search_color.clone());

    let mut to_add: Vec<String> = Vec::new();

    loop {

        let mut mod_count = 0;

        for search_bag in &directly_containing {
            for (color, contains) in rules {
                if color != search_bag && contains.contains_key(search_bag) {
                    to_add.push(color.clone());
                }
            }
        }

        while let Some(add_color) = to_add.pop() {
            if directly_containing.insert(add_color) {
                mod_count += 1;
            }
        }

        if mod_count == 0 {
            break;
        }

    }

    directly_containing.len() - 1

}

fn count_bags_into(rules: &BagRules, search_color: String) -> usize {
    count_bags_into_worker(rules, &search_color) - 1
}

fn count_bags_into_worker(rules: &BagRules, search_color: &String) -> usize {
    let mut total = 1;
    for (in_bag, count) in rules.get(search_color).unwrap() {
        total += count_bags_into_worker(rules, in_bag) * (*count as usize);
    }
    total
}

fn main() {

    let rules = parse_rules(INPUT.lines().map(String::from));
    let count = count_bags_containing(&rules, "shiny gold".to_string());
    let count_into = count_bags_into(&rules, "shiny gold".to_string());
    println!("Can contains count: {}", count);
    println!("Count into: {}", count_into);

}