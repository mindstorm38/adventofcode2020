use std::iter;

const INPUT: &str = include_str!("res/day5");

enum Half {
    Upper, Lower
}

impl Half {
    fn from_letter(c: char) -> Option<Self> {
        match c {
            'B' | 'R' => Some(Self::Upper),
            'F' | 'L' => Some(Self::Lower),
            _ => None
        }
    }
}

fn sep(range: &mut (u16, u16), half: Half) {
    let len = (range.1 - range.0) >> 1;
    match half {
        Half::Upper => { range.0 = range.1 - len },
        Half::Lower => { range.1 = range.0 + len; }
    };
}

fn parse_seats(inp: impl Iterator<Item=String>) -> Vec<(u16, u16)> {
    inp.map(|line| {

        let mut range = (0u16, 128u16);
        let mut chars = line.chars();
        let mut row = 0;

        for i in 0..10 {
            if i == 7 {
                row = range.0;
                range.0 = 0;
                range.1 = 8;
            }
            sep(&mut range, Half::from_letter(chars.next().unwrap()).unwrap());
        }

        (row, range.0)

    }).collect()
}

fn parse_seats_2(inp: impl Iterator<Item=String>) -> Vec<(u16, u16)> {
    inp.map(|line| {
        line.chars()
            .enumerate()
            .fold(((0u16, 0u16), (0u16, 128u16)), |mut acc, (idx, letter)| {
                if idx == 7 {
                    acc.0.0 = acc.1.0;
                    acc.1.0 = 0;
                    acc.1.1 = 8;
                }
                sep(&mut acc.1, Half::from_letter(letter).unwrap());
                if idx == 9 {
                    acc.0.1 = acc.1.0;
                }
                acc
            }).0
    }).collect()
}

fn main() {

    let seats = parse_seats_2(INPUT.lines().map(String::from));
    let bid = seats.iter().map(|&(row, col)| row * 8 + col).max().unwrap();

    let all_seats: Vec<bool> = seats.iter()
        .fold(vec![false; 128 * 8], |mut seats, &(row, col)| {
            seats[(row * 8 + col) as usize] = true;
            seats
        });

    let sid = all_seats.iter().enumerate()
        .skip_while(|(_, &b)| !b)
        .skip_while(|(_, &b)| b)
        .next().unwrap().0;

    println!("Max Board ID: {}", bid);
    println!("Your Board ID: {}", sid);

}