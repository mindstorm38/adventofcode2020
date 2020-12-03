
const INPUT: &str = include_str!("res/day3");
const PART2_SLOPES: [(u32, u32); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn traverse(mut inp: impl Iterator<Item = String>, slope: (u32, u32), mut pos: (u32, u32)) -> usize {

    let mut count = 0;

    'outer:
    while let Some(line) = inp.next() {

        if line.chars().nth(pos.0 as usize % line.len()).unwrap() == '#' {
            count += 1;
        }

        pos.0 += slope.0;
        pos.1 += slope.1;

        if slope.1 > 1 {
            for _ in 1..slope.1 {
                if let None = inp.next() {
                    break'outer;
                }
            }
        }

    }

    count

}

fn main() {


    println!("Part 1: {}", traverse(INPUT.lines().map(String::from), (3, 1), (0, 0)));

    let trees: usize = PART2_SLOPES.iter()
        .map(|slope| traverse(INPUT.lines().map(String::from), *slope, (0, 0)))
        .product();

    println!("Part 2: {}", trees);

}