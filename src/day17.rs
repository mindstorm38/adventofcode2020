use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("res/day17");

#[derive(Debug, Hash, Clone, Eq, PartialEq, Default)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

struct Grid {
    data: HashMap<Pos, bool>,
    active_count: u32,
    min_pos: Pos,
    max_pos: Pos
}

impl Grid {

    fn new() -> Grid {
        Grid {
            data: HashMap::new(),
            active_count: 0,
            min_pos: Pos::default(),
            max_pos: Pos::default()
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, w: i32, active: bool) {

        if x < self.min_pos.x {
            self.min_pos.x = x;
        } else if x > self.max_pos.x {
            self.max_pos.x = x;
        }

        if y < self.min_pos.y {
            self.min_pos.y = y;
        } else if y > self.max_pos.y {
            self.max_pos.y = y;
        }

        if z < self.min_pos.z {
            self.min_pos.z = z;
        } else if z > self.max_pos.z {
            self.max_pos.z = z;
        }

        if w < self.min_pos.w {
            self.min_pos.w = w;
        } else if w > self.max_pos.z {
            self.max_pos.w = w;
        }

        if active {
            self.active_count += 1;
        }

        if let Some(true) = self.data.insert(Pos { x, y, z, w }, active) {
            self.active_count -= 1;
        }

    }

    fn check_valid(&self, shared_pos: &Pos, min: u32, max: u32) -> bool {

        let ox = shared_pos.x;
        let oy = shared_pos.y;
        let oz = shared_pos.z;
        let ow = shared_pos.w;
        let mut cached_pos = shared_pos.clone();

        let mut count = 0;

        'all: for x in (ox - 1)..=(ox + 1) {
            cached_pos.x = x;
            for y in (oy - 1)..=(oy + 1) {
                cached_pos.y = y;
                for z in (oz - 1)..=(oz + 1) {
                    cached_pos.z = z;
                    for w in (ow - 1)..=(ow + 1) {
                        if x != ox || y != oy || z != oz || w != ow {
                            cached_pos.w = w;
                            if let Some(true) = self.data.get(&cached_pos) {
                                count += 1;
                                if count > max {
                                    break 'all;
                                }
                            }
                        }
                    }
                }
            }
        }

        min <= count && count <= max

    }

    fn step(&mut self, with_w: bool) {

        let mut changes: Vec<(i32, i32, i32, i32, bool)> = Vec::new();
        let mut shared_pos = Pos::default();

        for x in (self.min_pos.x - 1)..=(self.max_pos.x + 1) {
            shared_pos.x = x;
            for y in (self.min_pos.y - 1)..=(self.max_pos.y + 1) {
                shared_pos.y = y;
                for z in (self.min_pos.z - 1)..=(self.max_pos.z + 1) {
                    shared_pos.z = z;
                    if with_w {
                        for w in (self.min_pos.w - 1)..=(self.max_pos.w + 1) {
                            shared_pos.w = w;
                            self.step_at(&shared_pos, &mut changes);
                        }
                    } else {
                        self.step_at(&shared_pos, &mut changes);
                    }
                }
            }
        }

        for (x, y, z, w, active) in changes {
            self.set(x, y, z, w, active);
        }

    }

    fn step_at(&self, pos: &Pos, changes: &mut Vec<(i32, i32, i32, i32, bool)>) {
        if let Some(true) = self.data.get(&pos) {
            if !self.check_valid(&pos, 2, 3) {
                changes.push((pos.x, pos.y, pos.z, pos.w, false));
            }
        } else {
            if self.check_valid(&pos, 3, 3) {
                changes.push((pos.x, pos.y, pos.z, pos.w, true));
            }
        }
    }

    #[allow(unused)]
    fn debug(&self)  {

        let mut shared_pos = Pos::default();

        for z in self.min_pos.z..=self.max_pos.z {

            shared_pos.z = z;
            println!("z={}", z);

            for y in self.min_pos.y..=self.max_pos.y {
                shared_pos.y = y;
                for x in self.min_pos.x..=self.max_pos.x {
                    shared_pos.x = x;
                    if let Some(true) = self.data.get(&shared_pos) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!()
            }
            println!();
        }

    }

}

fn main() {

    let mut grid: Grid = Grid::new();

    for (i, line) in INPUT.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.set(j as i32, i as i32, 0, 0, ch == '#');
        }
    }

    let start = Instant::now();

    println!("Initial active cubes: {}", grid.active_count);
    for i in 0..6 {
        grid.step(false);
        println!("Cycle #{} active cubes: {}", i + 1, grid.active_count);
    }
    println!("Done in {}s", start.elapsed().as_secs_f64());

}