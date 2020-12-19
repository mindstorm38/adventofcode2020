use std::collections::HashMap;


const INPUT: &str = include_str!("res/day19test2");


#[derive(Debug)]
enum Rule {
    SubRules(Vec<Vec<u16>>),
    Root(char)
}


fn parse() -> (HashMap<u16, Rule>, Vec<&'static str>) {

    let mut lines = INPUT.lines();
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    while let Some(line) = lines.next() {

        if line.is_empty() {
            break;
        }

        let colon_idx = line.find(":").unwrap();
        let id: u16 = (&line[..colon_idx]).parse().unwrap();
        let rules_raw = &line[(colon_idx + 2)..];

        let mut rules_chars = rules_raw.chars();

        if let Some('"') = rules_chars.next() {
            if let Some(c) = rules_chars.next() {
                if let Some('"') = rules_chars.next() {
                    rules.insert(id, Rule::Root(c));
                    continue;
                }
            }
        }

        let mut sub_rules = Vec::new();

        for sub_rule in rules_raw.split(" | ") {
            let mut rule_ids = Vec::new();
            for rule_id in sub_rule.split_whitespace() {
                rule_ids.push(rule_id.parse().unwrap());
            }
            sub_rules.push(rule_ids);
        }

        rules.insert(id, Rule::SubRules(sub_rules));

    }

    messages.extend(lines);

    (rules, messages)

}


fn check(rules: &HashMap<u16, Rule>, message: &str) -> bool {
    println!("Checking '{}' ...", message);
    let res = check_worker(rules, 0, message);
    res.0 && res.1 == message.len()
}

fn check_worker(rules: &HashMap<u16, Rule>, rule_id: u16, message: &str) -> (bool, usize) {

    if message.is_empty() {
       return (false, 0);
    }

    let rule = &rules[&rule_id];

    let ret = match rule {
        Rule::Root(c) => {
            (message.chars().next().unwrap() == *c, 1)
        }
        Rule::SubRules(sub_rules) => {

            let mut ret = (false, 0);

            'sr: for sub_rule in sub_rules {

                let mut offset = 0;
                for &rule in sub_rule {
                    let (rule_res, rule_read) = check_worker(rules, rule, &message[offset..]);
                    if rule_res {
                        offset += rule_read;
                    } else {
                        continue'sr;
                    }
                }

                ret.0 = true;
                ret.1 = offset;
                break;

            }

            ret

        }
    };

    println!(" => {} with #{} {:?} : {}", message, rule_id, rule, ret.0);

    ret

}


fn main() {

    let (mut rules, messages) = parse();

    rules.insert(8, Rule::SubRules(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));

    for &msg in &messages {
        if check(&rules, msg) {
            println!("=> yes!");
        } else {
            println!("=> no!");
        }
    }

    //println!("#1 Valid count: {}", messages.iter().filter(|&&msg| check(&rules, msg)).count());


    //println!("#2 Valid count: {}", messages.iter().filter(|&&msg| check(&rules, msg)).count());

}