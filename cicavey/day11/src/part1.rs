use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    i: u64,
    x: i64,
    y: i64,
}

impl Galaxy {
    pub fn dist(&self, rhs: Galaxy) -> u64 {
        return ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as u64;
    }
    // const N: u8 = 1;
    // const S: u8 = 2;
    // const E: u8 = 4;
    // const W: u8 = 8;

    // pub fn new() -> Self {
    //     return Self {
    //         data: HashMap::new(),
    //     };
    // }

    // pub fn add(&mut self, x: i32, y: i32, v: (char, u8)) {
    //     self.data.insert(point2(x, y), v);
    // }

    // pub fn get(&self, p: Point2D<i32, AoC>) -> (char, u8) {
    //     return *self.data.get(&p).unwrap_or(&('.', 0));
    // }

    // pub fn look(&self, p: Point2D<i32, AoC>, dir: Vector2D<i32, AoC>) -> (char, u8) {
    //     return *self.data.get(&(p + dir)).unwrap_or(&('.', 0));
    // }
}

pub fn process(input: &str) -> String {
    let mut id_ctr = 1u64;
    let mut galaxy = vec![];
    let mut y_max = 0;
    let mut x_max = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        y_max = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            x_max = x as i64;
            if c == '.' {
                return;
            }
            let g = Galaxy {
                i: id_ctr,
                x: x as i64,
                y: y as i64,
            };
            galaxy.push(g);
            id_ctr += 1;
        });
    });

    // Expansion
    let mut expanded_temp = vec![];

    // Expand cols first
    let mut x_factor = 0;
    for x in 0..=x_max {
        // is empty col?
        if galaxy.iter().any(|g| g.x == x) {
            galaxy.iter().filter(|g| g.x == x).for_each(|g| {
                let mut eg = g.clone();
                eg.x += x_factor;
                expanded_temp.push(eg);
            })
        } else {
            x_factor += 1;
        }
    }

    let mut expanded = vec![];

    let mut y_factor = 0;
    for y in 0..=y_max {
        // is empty row?
        if expanded_temp.iter().any(|g| g.y == y) {
            expanded_temp.iter().filter(|g| g.y == y).for_each(|g| {
                let mut eg = g.clone();
                eg.y += y_factor;
                expanded.push(eg);
            })
        } else {
            y_factor += 1;
        }
    }

    let pathsum = expanded
        .iter()
        .combinations(2)
        .map(|v| v[0].dist(v[1].clone()))
        .sum::<u64>();

    return format!("{:?}", pathsum);
}
