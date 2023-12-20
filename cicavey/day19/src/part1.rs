use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
    accepted: bool,
}

impl Part {
    fn set(&mut self, k: &str, v: i64) {
        match k {
            "x" => self.x = v,
            "m" => self.m = v,
            "a" => self.a = v,
            "s" => self.s = v,
            _ => todo!(),
        }
    }

    fn get(&self, k: &str) -> i64 {
        match k {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => todo!(),
        }
    }

    fn total(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rules: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
enum Op {
    GT,
    LT,
    TERM,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    var: String,
    op: Op,
    val: i64,
    dst: String,
}

pub fn process(input: &str) -> String {
    let mut parts = vec![];
    let mut workflows = HashMap::new();
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }

        if l.starts_with("{") {
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
                accepted: false,
            };
            let v = l[1..l.len() - 1].split(",").for_each(|p| {
                let kv = p.split("=").collect_vec();
                part.set(kv[0], kv[1].parse().unwrap());
            });
            parts.push(part);
        } else {
            let a = l.split("{").collect_vec();
            let mut wf = Workflow::new(a[0]);
            let rule_str = a[1];
            rule_str[0..(rule_str.len() - 1)].split(",").for_each(|r| {
                if r.contains(":") {
                    let rv = r.split(":").collect_vec();
                    if rv[0].contains("<") {
                        let rvo = rv[0].split("<").collect_vec();
                        wf.rules.push(Rule {
                            var: rvo[0].to_string(),
                            op: Op::LT,
                            val: rvo[1].parse().unwrap(),
                            dst: rv[1].to_string(),
                        });
                    } else {
                        let rvo = rv[0].split(">").collect_vec();
                        wf.rules.push(Rule {
                            var: rvo[0].to_string(),
                            op: Op::GT,
                            val: rvo[1].parse().unwrap(),
                            dst: rv[1].to_string(),
                        });
                    }
                } else {
                    wf.rules.push(Rule {
                        var: "".to_string(),
                        op: Op::TERM,
                        val: 0,
                        dst: r.to_string(),
                    });
                }
            });
            workflows.insert(wf.name.clone(), wf);
        }
    }

    for p in parts.iter_mut() {
        let mut dst = "in".to_string();

        while dst != "A" && dst != "R" {
            let cur_wf = workflows.get(&dst).unwrap();
            for rule in &cur_wf.rules {
                if rule.op == Op::GT {
                    if p.get(&rule.var) > rule.val {
                        dst = rule.dst.to_string();
                        break;
                    } else {
                        continue;
                    }
                }
                if rule.op == Op::LT {
                    if p.get(&rule.var) < rule.val {
                        dst = rule.dst.to_string();
                        break;
                    } else {
                        continue;
                    }
                }
                dst = rule.dst.to_string();
            }
        }
        p.accepted = dst == "A";
    }

    let total: i64 = parts.iter().filter(|p| p.accepted).map(|p| p.total()).sum();

    return format!("{}", total);
}
