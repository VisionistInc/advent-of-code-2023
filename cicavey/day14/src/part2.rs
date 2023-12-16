use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Copy, Hash)]
enum State {
    Empty,
    Rolling,
    Fixed,
}

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            State::Empty => '.',
            State::Fixed => '#',
            State::Rolling => 'O',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Grid {
    data: Vec<State>,
    w: usize,
    h: usize,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            data: vec![State::Empty; w * h],
            w: w,
            h: h,
        }
    }

    fn get(&self, x: usize, y: usize) -> State {
        return self.data[y * self.h + x];
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.data[y * self.h + x] == State::Empty
    }

    fn set(&mut self, x: usize, y: usize, s: State) {
        self.data[y * self.h + x] = s;
    }

    fn clear(&mut self, x: usize, y: usize) {
        self.data[y * self.h + x] = State::Empty;
    }

    fn show(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", self.get(x, y));
            }
            println!();
        }
        println!();
    }

    fn load(&self) -> u64 {
        let mut sum = 0u64;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.get(x, y) == State::Rolling {
                    sum += self.h as u64 - y as u64;
                }
            }
        }
        return sum;
    }

    fn tilt_north(&self) -> Grid {
        let mut g2 = Grid::new(self.w, self.h);

        for y in 0..g2.h {
            for x in 0..g2.w {
                let s = self.get(x, y);
                if s == State::Empty || s == State::Fixed || y == 0 {
                    g2.set(x, y, s);
                    continue;
                }

                let mut t = y;
                while t > 0 && g2.is_empty(x, t - 1) {
                    t -= 1;
                }

                if t == y {
                    // no movement
                    g2.set(x, y, s);
                } else {
                    g2.set(x, t, s);
                    g2.clear(x, y);
                }
            }
        }

        return g2;
    }

    fn tilt_south(&self) -> Grid {
        let mut g2 = Grid::new(self.w, self.h);

        for y in (0..g2.h).rev() {
            for x in 0..g2.w {
                let s = self.get(x, y);
                if s == State::Empty || s == State::Fixed || y == self.h - 1 {
                    g2.set(x, y, s);
                    continue;
                }

                let mut t = y;
                while t < self.h - 1 && g2.is_empty(x, t + 1) {
                    t += 1;
                }

                if t == y {
                    // no movement
                    g2.set(x, y, s);
                } else {
                    g2.set(x, t, s);
                    g2.clear(x, y);
                }
            }
        }

        return g2;
    }

    fn tilt_west(&self) -> Grid {
        let mut g2 = Grid::new(self.w, self.h);

        for x in 0..g2.w {
            for y in 0..g2.h {
                let s = self.get(x, y);
                if s == State::Empty || s == State::Fixed || x == 0 {
                    g2.set(x, y, s);
                    continue;
                }

                let mut t = x;
                while t > 0 && g2.is_empty(t - 1, y) {
                    t -= 1;
                }

                if t == x {
                    // no movement
                    g2.set(x, y, s);
                } else {
                    g2.set(t, y, s);
                    g2.clear(x, y);
                }
            }
        }

        return g2;
    }

    fn tilt_east(&self) -> Grid {
        let mut g2 = Grid::new(self.w, self.h);

        for x in (0..g2.w).rev() {
            for y in 0..g2.h {
                let s = self.get(x, y);
                if s == State::Empty || s == State::Fixed || x == self.w - 1 {
                    g2.set(x, y, s);
                    continue;
                }

                let mut t = x;
                while t < self.w - 1 && g2.is_empty(t + 1, y) {
                    t += 1;
                }

                if t == x {
                    // no movement
                    g2.set(x, y, s);
                } else {
                    g2.set(t, y, s);
                    g2.clear(x, y);
                }
            }
        }

        return g2;
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn process(input: &str) -> String {
    let mut grid = Grid::new(100, 100);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.set(
                x,
                y,
                match c {
                    '#' => State::Fixed,
                    'O' => State::Rolling,
                    _ => State::Empty,
                },
            );
        }
    }

    let mut cache = HashMap::new();
    let mut cache_load = HashMap::new();
    let mut first = true;
    let mut loop_start = 0;
    let mut loop_delta = 0;

    for i in 1..1000 {
        grid = grid.tilt_north();
        grid = grid.tilt_west();
        grid = grid.tilt_south();
        grid = grid.tilt_east();

        println!("i = {}, load = {}", i, grid.load());

        let h = calculate_hash(&grid);

        if cache.contains_key(&h) {
            if first {
                first = false;
                loop_start = *cache.get(&h).unwrap();
                loop_delta = i - loop_start;
            }

            let r1 = loop_start + ((1_000_000_000u64 - i) % loop_delta) + 1;

            // let r1 = loop_start + (1_000_000_000u64 - 1 - loop_start) % loop_delta;

            // let loop_start = *cache.get(&h).unwrap();
            // let r1 = ((1_000_000_000u64 - loop_start) % loop_delta) + loop_start - 1;

            println!(
                "loop at {} back to {:?} ({} which is load {:?})",
                i,
                loop_start,
                r1,
                cache_load.get(&r1).unwrap()
            );

            return format!("{}", cache_load.get(&r1).unwrap());
        }

        cache.insert(h, i as u64);
        cache_load.insert(i as u64, grid.load());
    }

    return format!("{}", grid.load());
}
