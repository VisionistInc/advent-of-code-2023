use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

use image::{ImageBuffer, RgbImage};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn process(input: &str) -> String {
    let UP: Point2D = Point2D::new(0, -1);
    let DOWN: Point2D = Point2D::new(0, 1);
    let LEFT: Point2D = Point2D::new(-1, 0);
    let RIGHT: Point2D = Point2D::new(1, 0);
    let ZERO: Point2D = Point2D::new(0, 0);

    let mut cur = Point2D::new(0, 0);
    let mut grid = HashSet::new();

    for l in input.lines() {
        let s = l.split_whitespace().collect_vec();

        let v = match s[0] {
            "U" => UP,
            "D" => DOWN,
            "L" => LEFT,
            "R" => RIGHT,
            _ => ZERO,
        };
        let steps = s[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            cur = cur + v;
            grid.insert(cur);
        }
    }

    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0i32, 0i32, 0i32, 0i32);
    for p in grid.iter() {
        x_min = min(x_min, p.x);
        x_max = max(x_max, p.x);
        y_min = min(y_min, p.y);
        y_max = max(y_max, p.y);
    }

    let w: u32 = (x_max - x_min) as u32 + 1;
    let h: u32 = (y_max - y_min) as u32 + 1;

    let mut img: RgbImage = ImageBuffer::new(w, h);
    let white = image::Rgb([255, 255, 255]);

    for y in 0..h {
        for x in 0..w {
            let p = Point2D::new(x_min + x as i32, y_min + y as i32);
            if grid.contains(&p) {
                // print!("#")
                img.put_pixel(x, y, white);
            }
        }
    }

    let ORIGIN = Point2D::new(x_min, y_min);

    let mut q = VecDeque::new();
    q.push_back(Point2D::new((w / 2) as i32, (h / 2) as i32));

    while !q.is_empty() {
        let n = q.pop_front().unwrap();

        // Hit edge
        if grid.contains(&(n + ORIGIN)) {
            continue;
        }

        let px = img.get_pixel(n.x as u32, n.y as u32);
        if *px == white {
            continue;
        }

        img.put_pixel(n.x as u32, n.y as u32, white);

        q.push_back(n + UP);
        q.push_back(n + DOWN);
        q.push_back(n + LEFT);
        q.push_back(n + RIGHT);
    }

    let wcount = img.pixels().map(|p| *p == white).filter(|v| *v).count();

    img.save("test.png").unwrap();

    return format!("{}", wcount);
}
