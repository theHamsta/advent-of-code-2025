use std::collections::HashMap;

use itertools::Itertools;
//use regex::Regex;

fn main() -> anyhow::Result<()> {
    //let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    //let number_regex = Regex::new(r"(\d+)\n").unwrap();
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let letters: HashMap<_, _> = input
            .lines()
            .map(|l| {
                let mut it = l.split(':');
                (
                    it.next().unwrap(),
                    it.next()
                        .unwrap()
                        .split(' ')
                        .filter(|i| !i.is_empty())
                        .collect_vec(),
                )
            })
            .collect();
        dbg!(&letters);

        //for (from, targets) in letters.iter() {
        //for to in targets.iter() {
        //println!("{from} -> {to};");
        //}
        //}

        {
            let start = "you";
            let goal = "out";

            let mut path_count: HashMap<_, i64> = HashMap::new();

            let mut todo = vec![(1i64, start)];

            while let Some((count, cur)) = todo.pop() {
                let entry = path_count.entry(cur).or_default();
                *entry += count;

                if let Some(neighbors) = letters.get(cur) {
                    for n in neighbors.iter() {
                        todo.push((count, n));
                    }
                }
            }
            let part1 = path_count.get(goal).copied().unwrap_or_default();
            dbg!(part1);
        }

        let start = "svr";

        let mut part2 = 0;
        // second one impossible
        for routes in [["fft", "dac", "out"], ["dac", "fft", "out"]] {
            let mut prev = start;
            let mut prev_count = 1;

            for &stop in routes.iter() {
                let mut todo = vec![(prev_count, prev, prev)];
                let mut visited: HashMap<&str, Vec<&str>> = HashMap::new();

                let mut path_count: HashMap<_, i64> = HashMap::new();
                while let Some((count, cur, prev)) = todo.pop() {
                    let entry = path_count.entry(cur).or_default();
                    *entry += count;
                    let entry = visited.entry(cur);
                    match entry {
                        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                            occupied_entry.get_mut().push(prev)
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert_entry(vec![prev]);
                            if let Some(neighbors) = letters.get(cur) {
                                for n in neighbors.iter() {
                                    todo.push((count, n, cur));
                                }
                            }
                        }
                    }
                }

                loop {
                    let mut corrections = vec![];

                    for (&key, &count) in path_count.iter() {
                        let sum: i64 = visited[key].iter().map(|v| path_count[v]).sum();
                        if sum != count {
                            corrections.push((key, sum));
                        }
                    }
                    if corrections.is_empty() {
                        break;
                    }

                    for (key, value) in corrections {
                        path_count.insert(key, value);
                    }
                }

                let count = path_count.get(stop).copied().unwrap_or_default();

                prev = stop;
                prev_count = count;
            }
            part2 += prev_count;
        }
        dbg!(part2);
    }
    Ok(())
}
