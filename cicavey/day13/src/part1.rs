#[derive(Debug, Clone, PartialEq)]
struct Panel {
    rows: Vec<Vec<u8>>,
    rowbin: Vec<u32>,
    colbin: Vec<u32>,
}

impl Panel {
    pub fn new() -> Self {
        Panel {
            rows: vec![],
            rowbin: vec![],
            colbin: vec![],
        }
    }

    pub fn calc_colbin(&mut self) {
        let cols = self.rows[0].len();
        let nrow = self.rowbin.len();

        for col in 0..cols {
            let mut acc = 0;

            let mut c = nrow - 1;
            for r in &self.rows {
                if r[col] != 0 {
                    acc += 1 << c;
                }
                c -= 1;
            }

            self.colbin.push(acc);
        }
    }

    fn find_symmetry(v: &[u32]) -> usize {
        for i in 0..v.len() - 1 {
            let mut a = i as i32;
            let mut b = i as i32 + 1;

            while a > -1 && b < v.len() as i32 {
                if v[a as usize] == v[b as usize] {
                    a -= 1;
                    b += 1;
                } else {
                    a = -1;
                    b = -1;
                    break;
                }
            }

            // println!("{} {} {}", i, a, b);

            if a != b {
                // println!("potential {}", i + 1);
                return i + 1; // 1 based
            }
        }
        0
    }

    pub fn check(&self) -> (usize, usize) {
        let v = Panel::find_symmetry(self.colbin.as_slice());
        let h = Panel::find_symmetry(self.rowbin.as_slice());

        // println!("{} {}", v, h);

        (v, h)
    }
}

pub fn process(input: &str) -> String {
    let mut panels = vec![];
    let mut p = Panel::new();

    for line in input.lines() {
        if line.is_empty() {
            p.calc_colbin();
            panels.push(p);
            p = Panel::new();
            continue;
        }

        let row = line
            .chars()
            .map(|c| match c {
                '#' => 1,
                _ => 0,
            })
            .collect::<Vec<u8>>();

        p.rows.push(row.clone());

        let rowbin = row
            .iter()
            .rev()
            .enumerate()
            .fold(0u32, |acc, e| acc + ((*e.1 as u32) << (e.0)));

        p.rowbin.push(rowbin);
    }

    let summary = panels
        .iter()
        .map(|p| p.check())
        .fold((0, 0), |acc, (v, h)| (acc.0 + v, acc.1 + h));

    return format!("{:?}", summary.0 + summary.1 * 100);
}
