use itertools::Itertools;

fn hash(s: &str) -> u64 {
    s.chars()
        .fold(0u64, |acc, c| ((acc + (c as u64)) * 17) % 256)
}

#[derive(Debug, Clone, PartialEq)]
struct Lens {
    label: String,
    focal_length: u64,
}

pub fn process(input: &str) -> String {
    let mut boxes: Vec<Vec<Lens>> = vec![];
    (0..255).for_each(|_| boxes.push(vec![]));

    input.split(",").for_each(|v| {
        if v.contains("=") {
            let (label, lens) = v.split("=").collect_tuple().unwrap();
            let box_num = hash(label);
            let focal_length = lens.parse::<u64>().unwrap();
            let lens = Lens {
                label: label.to_string(),
                focal_length: focal_length,
            };

            let b = &mut boxes[box_num as usize];

            if b.iter().any(|l| l.label == lens.label) {
                b.iter_mut().for_each(|l| {
                    if *l.label == lens.label {
                        *l = Lens {
                            label: lens.label.clone(),
                            focal_length: lens.focal_length,
                        };
                    }
                });
            } else {
                b.push(lens);
            }

            // println!("hhh {} {} box {}", label, focal_length, box_num);
        } else {
            let (label, _) = v.split("-").collect_tuple().unwrap();
            let box_num = hash(label);

            let b = &mut boxes[box_num as usize];
            b.retain(|l| l.label != label);

            // println!("rem {} box {}", label, box_num);
        }

        // boxes
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, v)| v.len() > 0)
        //     .for_each(|(i, v)| println!("box {} : {:?}", i, v));
        // println!();
    });

    let focusing_power: u64 = boxes
        .iter()
        .enumerate()
        .filter(|(_, v)| v.len() > 0)
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(s, l)| (i as u64 + 1) * (s as u64 + 1) * l.focal_length)
                .sum::<u64>()
        })
        .sum();

    return format!("{}", focusing_power);
}
