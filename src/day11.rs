use std::cmp::min;
use std::time::Instant;

const INPUT: &str = include_str!("res/day11");

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Pos {
    Floor,
    EmptySeat,
    OccupiedSeat
}

struct Layout {
    grid: Vec<Pos>,
    width: usize,
    height: usize
}

impl Pos {

    fn from_char(ch: char) -> Pos {
        use Pos::*;
        match ch {
            'L' => EmptySeat,
            '#' => OccupiedSeat,
            _ => Floor
        }
    }

}

impl Layout {

    fn from_lines(inp: impl Iterator<Item = String>) -> Layout {

        let mut layout = Layout {
            grid: Vec::new(),
            width: 0,
            height: 0
        };

        for (i, line) in inp.enumerate() {
            layout.grid.extend(line.chars().map(Pos::from_char));
            layout.height += 1;
            if i == 0 {
                layout.width = layout.grid.len();
            }
        }

        layout

    }

    fn count_pos(&self, pos: Pos) -> usize {
        self.grid.iter().filter(move |&&p| p == pos).count()
    }

    fn count_adjacent(&self, x: usize, y: usize, pos: Pos) -> usize {
        let from_x = if x == 0 { 0 } else { x - 1 };
        let from_y = if y == 0 { 0 } else { y - 1 };
        let to_x = min(x + 1, self.width - 1);
        let to_y = min(y + 1, self.height - 1);
        let mut count = 0usize;
        for dx in from_x..=to_x {
            for dy in from_y..=to_y {
                if (dx != x || dy != y) && self.grid[dx + dy * self.width] == pos {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_adjacent_long(&self, x: usize, y: usize, pos: Pos) -> usize {
        let mut count = 0usize;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 || dy != 0) && self.count_adjacent_vec(x as isize + dx, y as isize + dy, dx, dy, pos) {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_adjacent_vec(&self, mut x: isize, mut y: isize, dx: isize, dy: isize, pos: Pos) -> bool {
        //println!("## Count {}/{} with {}/{}", x, y, dx, dy);
        let w = self.width as isize;
        let h = self.height as isize;
        let mut found = false;
        while x >= 0 && y >= 0 && x < w && y < h {
            //print!("{}/{} ", x, y);
            let curr_pos = self.grid[x as usize + y as usize * self.width];
            if curr_pos != Pos::Floor {
                found = curr_pos == pos;
                break;
            } else {
                x += dx;
                y += dy;
            }
        }
        //println!();
        found
    }

    fn do_step<F>(&mut self, count_func: F, occupied_limit: usize) -> usize
        where F: Fn(&Self, usize, usize, Pos) -> usize {

        let mut changes: Vec<(usize, Pos)> = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let idx = x + y * self.width;
                match self.grid[idx] {
                    Pos::EmptySeat => {
                        if count_func(self, x, y, Pos::OccupiedSeat) == 0 {
                            changes.push((idx, Pos::OccupiedSeat));
                        }
                    },
                    Pos::OccupiedSeat => {
                        if count_func(self, x, y, Pos::OccupiedSeat) >= occupied_limit {
                            changes.push((idx, Pos::EmptySeat));
                        }
                    },
                    _ => {}
                }
            }
        }

        for &(idx, new_pos) in &changes {
            self.grid[idx] = new_pos;
        }

        changes.len()

    }

    #[allow(unused)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", match self.grid[x + y * self.width] {
                    Pos::Floor => '.',
                    Pos::EmptySeat => 'L',
                    Pos::OccupiedSeat => '#'
                });
            }
            println!();
        }
    }

}


fn main() {

    let start = Instant::now();
    let mut layout = Layout::from_lines(INPUT.lines().map(String::from));
    while layout.do_step(Layout::count_adjacent, 4) != 0 { }
    println!("[#1] Occupied count: {} (in {}ms)", layout.count_pos(Pos::OccupiedSeat), start.elapsed().as_secs_f64() * 1000f64);

    let start = Instant::now();
    let mut layout = Layout::from_lines(INPUT.lines().map(String::from));
    while layout.do_step(Layout::count_adjacent_long, 5) != 0 { }
    println!("[#2] Occupied count: {} (in {}ms)", layout.count_pos(Pos::OccupiedSeat), start.elapsed().as_secs_f64() * 1000f64);

}