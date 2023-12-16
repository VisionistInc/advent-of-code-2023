use std::fmt;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Copy)]
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

                let mut ty = y;
                while ty > 0 && g2.is_empty(x, ty - 1) {
                    ty -= 1;
                }

                if ty == y {
                    // no movement
                    g2.set(x, y, s);
                } else {
                    g2.set(x, ty, s);
                    g2.clear(x, y);
                }
            }
        }

        return g2;
    }
}

pub fn process(input: &str) -> String {
    let mut grid = Grid::new(10, 10);

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

    let g2 = grid.tilt_north();

    g2.show();

    return format!("{}", g2.load());
}
