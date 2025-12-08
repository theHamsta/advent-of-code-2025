use std::collections::HashMap;

use itertools::Itertools;
//use regex::Regex;

fn distance(a: [i64; 3], b: [i64; 3]) -> i64 {
    (0..a.len()).map(|i| (a[i] - b[i]) * (a[i] - b[i])).sum()
}

fn main() -> anyhow::Result<()> {
    //let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    //let number_regex = Regex::new(r"(\d+)\n").unwrap();
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let numbers: Vec<[i64; 3]> = input
            .lines()
            .flat_map(|l| {
                l.split(',')
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect_array()
            })
            .filter(|l| !l.is_empty())
            .collect_vec();

        let mut index = 0;
        let mut connections = HashMap::new();

        let closest = numbers
            .iter()
            .copied()
            .tuple_combinations()
            .sorted_by_key(|(a, b)| distance(*a, *b))
            .take(1000);

        for (a, b) in closest {
            let a_index = *connections.entry(a).or_insert_with(|| {
                let rtn = index;
                index += 1;
                rtn
            });
            let b_index = *connections.entry(b).or_insert_with(|| {
                let rtn = index;
                index += 1;
                rtn
            });

            if a_index == b_index {
                continue;
            } else {
                for (_, v) in connections.iter_mut() {
                    if *v == a_index {
                        *v = b_index;
                    }
                }
            }
        }

        let part1: usize = connections
            .values()
            .counts()
            .values()
            .sorted()
            .rev()
            .take(3)
            .product();
        dbg!(part1);

        let mut part2 = None;
        let closest = numbers
            .iter()
            .copied()
            .tuple_combinations()
            .sorted_by_key(|(a, b)| distance(*a, *b));
        let mut connections: HashMap<_, _> = numbers
            .iter()
            .copied()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect();

        for (a, b) in closest {
            let a_index = *connections.entry(a).or_insert_with(|| {
                let rtn = index;
                index += 1;
                rtn
            });
            let b_index = *connections.entry(b).or_insert_with(|| {
                let rtn = index;
                index += 1;
                rtn
            });

            if a_index == b_index {
                continue;
            } else {
                for (_, v) in connections.iter_mut() {
                    if *v == a_index {
                        *v = b_index;
                    }
                }
            }

            if connections.values().unique().count() == 1 {
                part2 = Some(a[0] * b[0]);
            }
        }
        dbg!(part2.unwrap());
    }
    Ok(())
}
