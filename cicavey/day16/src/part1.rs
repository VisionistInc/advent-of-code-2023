use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
enum State {
    Empty,
    ForwardSlash,
    BackSlash,
    Horz,
    Vert,
}

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            State::Empty => '.',
            State::ForwardSlash => '/',
            State::BackSlash => '\\',
            State::Horz => '-',
            State::Vert => '|',
        };
        write!(f, "{}", c)
    }
}

struct Grid {
    data: Vec<State>,
    w: i16,
    h: i16,
}

impl Grid {
    fn new(w: i16, h: i16) -> Self {
        Self {
            data: vec![State::Empty; (w * h) as usize],
            w: w,
            h: h,
        }
    }

    fn get(&self, x: i16, y: i16) -> State {
        return self.data[(y * self.h + x) as usize];
    }

    fn is_empty(&self, x: i16, y: i16) -> bool {
        self.data[(y * self.h + x) as usize] == State::Empty
    }

    fn set(&mut self, x: i16, y: i16, s: State) {
        self.data[(y * self.h + x) as usize] = s;
    }

    fn show(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }

    fn contains(&self, x: i16, y: i16) -> bool {
        x >= 0 && y >= 0 && x < self.w && y < self.h
    }
}

struct VisitGrid {
    data: Vec<u8>,
    w: i16,
    h: i16,
}

impl VisitGrid {
    fn new(w: i16, h: i16) -> Self {
        Self {
            data: vec![0; (w * h) as usize],
            w: w,
            h: h,
        }
    }

    fn get(&self, x: i16, y: i16) -> u8 {
        return self.data[(y * self.h + x) as usize];
    }

    fn set(&mut self, x: i16, y: i16, s: u8) {
        self.data[(y * self.h + x) as usize] = s;
    }

    fn visit(&mut self, b: Beam) {
        let mut v = 0;
        if b.v.0 > 0 {
            v = 1;
        } else if b.v.0 < 0 {
            v = 2;
        } else if b.v.1 > 0 {
            v = 4;
        } else if b.v.1 < 0 {
            v = 8;
        }

        self.set(b.x, b.y, self.get(b.x, b.y) | v);
    }

    fn visited(&mut self, b: Beam) -> bool {
        let mut v = 0;
        if b.v.0 > 0 {
            v = 1;
        } else if b.v.0 < 0 {
            v = 2;
        } else if b.v.1 > 0 {
            v = 4;
        } else if b.v.1 < 0 {
            v = 8;
        }

        let dst = self.get(b.x, b.y);

        return dst & v != v;
    }

    fn show(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let v = self.get(x, y);
                if v != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Beam {
    x: i16,
    y: i16,
    v: (i16, i16),
    alive: bool,
}

impl Beam {
    fn step(&mut self) {
        self.x += self.v.0;
        self.y += self.v.1;
    }

    fn is_horz(&self) -> bool {
        self.v.1 == 0
    }

    fn is_vert(&self) -> bool {
        self.v.0 == 0
    }
}

pub fn process(input: &str) -> String {
    let mut grid = Grid::new(110, 110);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.set(
                x as i16,
                y as i16,
                match c {
                    '/' => State::ForwardSlash,
                    '\\' => State::BackSlash,
                    '-' => State::Horz,
                    '|' => State::Vert,
                    _ => State::Empty,
                },
            );
        }
    }

    let mut visited = VisitGrid::new(grid.w, grid.h);

    let mut beams = vec![Beam {
        x: 0,
        y: 0,
        v: (1, 0),
        alive: true,
    }];

    for step in 0..1000000 {
        let mut new_beams = vec![];
        for b in beams.iter_mut() {
            visited.visit(*b);
            let g = grid.get(b.x, b.y);
            match g {
                State::Empty => b.step(),
                State::ForwardSlash => {
                    // there is probably a mathy way to rotate vectors
                    if b.is_horz() {
                        b.v = (b.v.1, -b.v.0);
                    } else {
                        b.v = (-b.v.1, b.v.0);
                    }
                    b.step();
                }
                State::BackSlash => {
                    // there is probably a mathy way to rotate vectors
                    if b.is_horz() {
                        b.v = (b.v.1, b.v.0);
                    } else {
                        b.v = (b.v.1, b.v.0);
                    }
                    b.step();
                }
                State::Horz => {
                    if b.is_vert() {
                        // Alter v
                        b.v = (b.v.1, b.v.0);

                        let mut b2 = b.clone();
                        b2.v = (-b2.v.0, b2.v.1);

                        b.step();
                        b2.step();

                        new_beams.push(b2);
                    } else {
                        b.step();
                    }
                }
                State::Vert => {
                    if b.is_horz() {
                        // Alter v
                        b.v = (b.v.1, b.v.0);

                        let mut b2 = b.clone();
                        b2.v = (b2.v.0, -b2.v.1);

                        b.step();
                        b2.step();

                        new_beams.push(b2);
                    } else {
                        b.step();
                    }
                }
                _ => {}
            }
        }

        beams.append(&mut new_beams);

        // Cull beams outside the bounds
        beams.retain(|b| grid.contains(b.x, b.y));

        beams.iter_mut().for_each(|b| {
            // print!("has B {:?} visited in this direction? ", b);
            // println!("{} {}", visited.get(b.x, b.y), visited.visited(*b));
            b.alive = visited.visited(*b);
        });

        beams.retain(|b| b.alive);

        if beams.is_empty() {
            break;
        }
    }

    let c = visited.data.iter().map(|v| *v != 0).filter(|v| *v).count();

    return format!("{}", c);
}
