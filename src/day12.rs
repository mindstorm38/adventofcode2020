use std::time::Instant;

const INPUT: &str = include_str!("res/day12");

#[repr(u8)]
#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32)
}

impl From<&str> for Action {
    fn from(from: &str) -> Self {
        use Action::*;
        let (act, num) = from.split_at(1);
        let act = act.chars().next().unwrap();
        let num: i32 = num.parse().unwrap();
        match act {
            'N' => North(num),
            'S' => South(num),
            'E' => East(num),
            'W' => West(num),
            'L' => Left(num),
            'R' => Right(num),
            'F' => Forward(num),
            _ => unreachable!()
        }
    }
}


fn parse_actions(inp: impl Iterator<Item = String>) -> Vec<Action> {
    inp.map(|s| Action::from(s.as_str())).collect()
}

fn run_actions(actions: &Vec<Action>, waypoint: bool) -> (i32, i32) {

    const NORTH: i8 = 0;
    const EAST: i8 = 1;
    const SOUTH: i8 = 2;
    const WEST: i8 = 3;

    let mut facing = EAST;
    let mut east = 0i32; // East (+) / West (-)
    let mut north = 0i32; // North (+) / South (-)

    let mut wp_east = 10i32;
    let mut wp_north = 1i32;

    for action in actions {
        if waypoint {
            match *action {
                Action::North(d) => wp_north += d,
                Action::South(d) => wp_north -= d,
                Action::East(d) => wp_east += d,
                Action::West(d) => wp_east -= d,
                Action::Left(r) => for _ in 0..(r / 90) {
                    let tmp = wp_east;
                    wp_east = -wp_north;
                    wp_north = tmp;
                },
                Action::Right(r) => for _ in 0..(r / 90) {
                    let tmp = wp_east;
                    wp_east = wp_north;
                    wp_north = -tmp;
                },
                Action::Forward(d) => {
                    east += wp_east * d;
                    north += wp_north * d;
                }
            }
        } else {
            match *action {
                Action::North(d) => north += d,
                Action::South(d) => north -= d,
                Action::East(d) => east += d,
                Action::West(d) => east -= d,
                Action::Left(r) => facing = (facing - (r / 90) as i8).rem_euclid(4),
                Action::Right(r) => facing = (facing + (r / 90) as i8).rem_euclid(4),
                Action::Forward(d) => match facing {
                    NORTH => north += d,
                    SOUTH => north -= d,
                    EAST => east += d,
                    WEST => east -= d,
                    _ => unreachable!()
                }
            }
        }
    }

    (east, north)

}


fn main() {

    let start_global = Instant::now();

    let start = Instant::now();
    let actions = parse_actions(INPUT.lines().map(String::from));
    println!("Parsed in {}ms", start.elapsed().as_secs_f64() * 1000.0);
    println!();

    let start = Instant::now();
    let (east, north) = run_actions(&actions, false);
    println!("#1 Ending at {}/{}", east, north);
    println!("#1 Manhattan distance: {}", east.abs() + north.abs());
    println!("#1 in {}ms", start.elapsed().as_secs_f64() * 1000.0);
    println!();

    let start = Instant::now();
    let (east, north) = run_actions(&actions, true);
    println!("#2 Ending at {}/{}", east, north);
    println!("#2 Manhattan distance: {}", east.abs() + north.abs());
    println!("#2 in {}ms", start.elapsed().as_secs_f64() * 1000.0);
    println!();

    println!("Done in {}ms", start_global.elapsed().as_secs_f64() * 1000.0);


}