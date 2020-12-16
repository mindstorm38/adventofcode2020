use std::ops::RangeInclusive;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("res/day16");


#[derive(Debug)]
struct Ranges(RangeInclusive<u32>, RangeInclusive<u32>);


impl Ranges {
    fn contains(&self, val: u32) -> bool {
        self.0.contains(&val) || self.1.contains(&val)
    }
}


fn parse_range(raw: &str) -> RangeInclusive<u32> {
    let dash_idx = raw.find('-').unwrap();
    return raw[..dash_idx].parse().unwrap()..=raw[dash_idx + 1..].parse().unwrap();
}

fn parse_ticket(raw: &str) -> Vec<u32> {
    raw.split(",").map(|n| n.parse().unwrap()).collect()
}


fn parse() -> (HashMap<String, Ranges>, Vec<u32>, Vec<Vec<u32>>) {

    let mut lines = INPUT.lines();
    let mut fields: HashMap<String, Ranges> = HashMap::new();
    let mut tickets: Vec<Vec<u32>> = Vec::new();

    while let Some(line) = lines.next() {

        if line.is_empty() {
            break;
        } else {

            let colon_idx = line.find(':').unwrap();
            let field = &line[..colon_idx];
            let ranges = &line[colon_idx + 2..];

            let or_idx = ranges.find("or").unwrap();

            fields.insert(field.to_string(), Ranges(
                parse_range(&ranges[..or_idx - 1]),
                parse_range(&ranges[or_idx + 3..])
            ));

        }

    }

    lines.next();
    let your_ticket = parse_ticket(lines.next().unwrap());

    lines.next();
    lines.next();

    while let Some(line) = lines.next() {
        tickets.push(parse_ticket(line));
    }

    (fields, your_ticket, tickets)

}

fn filter_valid_tickets(fields: &HashMap<String, Ranges>, raw: Vec<Vec<u32>>) -> (u32, Vec<Vec<u32>>) {

    let mut error_rate = 0u32;
    let mut valid_tickets: Vec<Vec<u32>> = Vec::new();

    for ticket in raw {
        let mut found = false;
        for &num in &ticket {
            found = false;
            for ranges in fields.values() {
                if ranges.contains(num) {
                    found = true;
                    break;
                }
            }
            if !found {
                error_rate += num;
                break;
            }
        }
        if found {
            valid_tickets.push(ticket);
        }
    }

    (error_rate, valid_tickets)

}

fn find_valid_fields(fields: &HashMap<String, Ranges>, valid_tickets: Vec<Vec<u32>>) -> Vec<(usize, &String)> {

    let fields_count = valid_tickets[0].len();
    let mut pending_fields: Vec<(usize, Vec<&String>)> = Vec::new();
    let mut valid_fields: Vec<(usize, &String)> = Vec::new();

    for i in 0..fields_count {

        let mut available_fields: Vec<&String> = fields.keys().collect();

        for ticket in &valid_tickets {
            let val = ticket[i];
            available_fields.retain(|&t| {
                (&fields[t]).contains(val)
            });
        }

        if available_fields.len() == 1 {
            valid_fields.push((i, available_fields[0]));
        } else {
            pending_fields.push((i, available_fields));
        }

    }

    while valid_fields.len() != fields_count {

        let valid_field = valid_fields.last().unwrap().1;
        let mut to_remove_idx = 0;

        for i in 0..pending_fields.len() {

            let (j, fields) = &mut pending_fields[i];
            fields.remove(fields.iter().position(|&field| field == valid_field).unwrap());

            if fields.len() == 1 {
                to_remove_idx = i;
                valid_fields.push((*j, fields[0]));
            }

        }

        pending_fields.remove(to_remove_idx);

    }

    valid_fields

}

fn main() {

    let start_global = Instant::now();

    let start = Instant::now();
    let (
        fields,
        your_ticket,
        tickets
    ) = parse();
    println!("Parsed in {}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = Instant::now();
    let (
        error_rate,
        valid_tickets
    ) = filter_valid_tickets(&fields, tickets);
    println!("#1 Filtered tickets in {}ms", start.elapsed().as_secs_f64() * 1000.0);
    println!("#1 Error rate: {}", error_rate);

    let start = Instant::now();
    let valid_fields = find_valid_fields(&fields, valid_tickets);
    println!("#2 Found valid fields in {}ms", start.elapsed().as_secs_f64() * 1000.0);

    let result: u64 = valid_fields.iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|(idx, _)| your_ticket[*idx] as u64)
        .product();

    println!("#2 Valid fields: {:?}", valid_fields);
    println!("#2 Result: {}", result);

    println!("All in {}ms", start_global.elapsed().as_secs_f64() * 1000.0);

}