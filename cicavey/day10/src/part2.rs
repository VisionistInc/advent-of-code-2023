use euclid::{point2, vec2, Point2D, Vector2D};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum AoC {}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    data: HashMap<Point2D<i32, AoC>, (char, u8)>,
}

impl Grid {
    const N: u8 = 1;
    const S: u8 = 2;
    const E: u8 = 4;
    const W: u8 = 8;

    pub fn new() -> Self {
        return Self {
            data: HashMap::new(),
        };
    }

    pub fn add(&mut self, x: i32, y: i32, v: (char, u8)) {
        self.data.insert(point2(x, y), v);
    }

    pub fn get(&self, p: Point2D<i32, AoC>) -> (char, u8) {
        return *self.data.get(&p).unwrap_or(&('.', 0));
    }

    pub fn look(&self, p: Point2D<i32, AoC>, dir: Vector2D<i32, AoC>) -> (char, u8) {
        return *self.data.get(&(p + dir)).unwrap_or(&('.', 0));
    }
}

pub fn process(input: &str) -> String {
    const up: Vector2D<i32, AoC> = vec2(0, -1);
    const down: Vector2D<i32, AoC> = vec2(0, 1);
    const right: Vector2D<i32, AoC> = vec2(1, 0);
    const left: Vector2D<i32, AoC> = vec2(-1, 0);

    let mut grid = Grid::new();

    let mut start: Point2D<i32, AoC> = point2(0, 0);

    let mut max_x = 0;
    let mut max_y = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        max_y = y as i32;
        line.chars().enumerate().for_each(|(x, c)| {
            max_x = x as i32;
            if c == '.' {
                return;
            }

            let exits = match c {
                '|' => Grid::N + Grid::S,
                '-' => Grid::E + Grid::W,
                'L' => Grid::N + Grid::E,
                'J' => Grid::N + Grid::W,
                '7' => Grid::S + Grid::W,
                'F' => Grid::S + Grid::E,
                _ => 0,
            };

            grid.add(x as i32, y as i32, (c, exits));
            if c == 'S' {
                start = point2(x as i32, y as i32);
            }
        });
    });

    // Resolve start
    let exit_N = grid.look(start, up).1 & Grid::S;
    let exit_S = grid.look(start, down).1 & Grid::N;
    let exit_W = grid.look(start, left).1 & Grid::E;
    let exit_E = grid.look(start, right).1 & Grid::W;
    let mut exits = 0;
    if exit_N != 0 {
        exits += Grid::N;
    }
    if exit_S != 0 {
        exits += Grid::S;
    }
    if exit_E != 0 {
        exits += Grid::E;
    }
    if exit_W != 0 {
        exits += Grid::W;
    }

    grid.add(start.x, start.y, ('S', exits));

    let mut steps = 0u64;
    let mut cur = start;
    let mut prev = start;

    // track loop points for part 2

    let mut inloop: HashSet<Point2D<i32, AoC>> = HashSet::new();

    loop {
        inloop.insert(cur);
        if steps != 0 && cur == start {
            break;
        }

        let (_c, exits) = grid.get(cur);
        let mut dir = up;

        if exits & Grid::N == Grid::N {
            if cur + up != prev {
                dir = up;
            }
        }
        if exits & Grid::S == Grid::S {
            if cur + down != prev {
                dir = down;
            }
        }
        if exits & Grid::W == Grid::W {
            if cur + left != prev {
                dir = left;
            }
        }
        if exits & Grid::E == Grid::E {
            if cur + right != prev {
                dir = right;
            }
        }

        prev = cur;
        cur = cur + dir;
        steps += 1;
    }

    // Ray casting algorithm
    // https://en.wikipedia.org/wiki/Point_in_polygon

    let mut enclosed = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let p = point2(x, y);

            if inloop.contains(&p) {
                let (og, _) = grid.data.get(&p).unwrap();
                print!("{}", &og);
                continue;
            }

            let mut rcount = 0;
            for i in x + 1..=max_x {
                let p2 = point2(i, y);
                if inloop.contains(&p2) {
                    let (og, _) = grid.data.get(&p2).unwrap();

                    // try just north-ish grid cells? kinda like a N rounding?
                    // FORGOT 'S', arg
                    if *og == '|' || *og == 'J' || *og == 'L' || *og == 'S' {
                        rcount += 1;
                    }
                }
            }

            if rcount % 2 == 0 {
                print!(".");
            } else {
                print!("I");
                enclosed += 1;
            }
        }
        println!();
    }

    return format!("{}", enclosed);
}
