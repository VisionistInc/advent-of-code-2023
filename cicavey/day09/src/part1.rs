fn diff(h: &[i64]) -> i64 {
    let next: Vec<_> = h.windows(2).map(|f| f[1] - f[0]).collect();
    let z = next.iter().all(|v| *v == 0);

    if z {
        return 0;
    } else {
        return next.last().unwrap() + diff(next.as_slice());
    }
}

fn predict(h: &[i64]) -> i64 {
    return h.last().unwrap() + diff(h);
}

pub fn process(input: &str) -> String {
    let mut histories = vec![];

    for line in input.lines() {
        let vals: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        histories.push(vals);
    }

    let a: i64 = histories.iter().map(|h| predict(h.as_slice())).sum();

    return format!("{}", a);
}
