use std::collections::HashMap;

use itertools::Itertools;
//use regex::Regex;

#[allow(dead_code)]
fn plot(x_max: i64, y_max: i64, positions: &HashMap<(i64, i64), bool>) {
    for y in 0..=y_max {
        for x in 0..x_max {
            if positions.contains_key(&(x, y)) {
                print!("{}", if positions[&(x, y)] { "." } else { "X" });
            } else {
                print!("Y");
            }
        }
        println!();
    }
    println!();
}

fn main() -> anyhow::Result<()> {
    //let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    //let number_regex = Regex::new(r"(\d+)\n").unwrap();
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let mut numbers: Vec<[i64; 2]> = input
            .lines()
            .flat_map(|l| {
                l.split(",")
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect_array()
            })
            .filter(|l| !l.is_empty())
            .collect_vec();

        let part1 = numbers
            .iter()
            .tuple_combinations()
            .map(|([ax, ay], [bx, by])| (ax.abs_diff(*bx) + 1) * (ay.abs_diff(*by) + 1))
            .max();
        dbg!(part1);

        let mut x_sorted = numbers
            .iter()
            .flat_map(|n| [n[0] - 1, n[0], n[0] + 1])
            .unique()
            .collect_vec();
        x_sorted.sort_unstable();

        let mut y_sorted = numbers
            .iter()
            .flat_map(|n| [n[1] - 1, n[1], n[1] + 1])
            .unique()
            .collect_vec();
        y_sorted.sort_unstable();

        let x_ranges = x_sorted
            .iter()
            .copied()
            .tuple_windows()
            .map(|(start, end)| start..end)
            .collect_vec();
        let y_ranges = y_sorted
            .iter()
            .copied()
            .tuple_windows()
            .map(|(start, end)| start..end)
            .collect_vec();

        numbers.push(numbers[0]);

        let mut all_green = HashMap::new();

        for (&[ax, ay], &[bx, by]) in numbers.iter().tuple_windows() {
            assert!(ax == bx || ay == by);

            let index_x = x_ranges.iter().position(|r| r.contains(&ax)).unwrap();
            let index_y = y_ranges.iter().position(|r| r.contains(&ay)).unwrap();
            all_green.insert((index_x as i64, index_y as i64), true);
            let index_x = x_ranges.iter().position(|r| r.contains(&bx)).unwrap();
            let index_y = y_ranges.iter().position(|r| r.contains(&by)).unwrap();
            all_green.insert((index_x as i64, index_y as i64), true);

            if ay == by {
                let mut index_x = x_ranges
                    .iter()
                    .position(|r| r.contains(&(ax.min(bx) + 1)))
                    .unwrap();
                let index_y = y_ranges.iter().position(|r| r.contains(&ay)).unwrap();

                while x_ranges[index_x].end <= ax.max(bx) {
                    all_green.insert((index_x as i64, index_y as i64), true);
                    index_x += 1;
                }
            } else {
                let index_x = x_ranges.iter().position(|r| r.contains(&ax)).unwrap();
                let mut index_y = y_ranges
                    .iter()
                    .position(|r| r.contains(&(ay.min(by) + 1)))
                    .unwrap();

                while y_ranges[index_y].end <= ay.max(by) {
                    all_green.insert((index_x as i64, index_y as i64), true);
                    index_y += 1;
                }
            }
        }

        let mut todo = vec![(0, 0)];

        while let Some((x, y)) = todo.pop() {
            if all_green.contains_key(&(x, y))
                || x >= x_sorted.len() as i64
                || y >= y_sorted.len() as i64
                || x < 0
                || y < 0
            {
                continue;
            }
            all_green.insert((x, y), false);
            todo.push((x - 1, y));
            todo.push((x + 1, y));
            todo.push((x, y + 1));
            todo.push((x, y - 1));
        }

        //plot(x_ranges.len() as i64, y_ranges.len() as i64, &all_green);

        let part2 = numbers
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|&([ax, ay], [bx, by])| {
                let min_x = ax.min(bx);
                let min_y = ay.min(by);
                let max_x = ax.max(bx);
                let max_y = ay.max(by);
                let index_x_min = x_ranges.iter().position(|r| r.contains(&min_x)).unwrap() as i64;
                let index_x_max = x_ranges.iter().position(|r| r.contains(&max_x)).unwrap() as i64;
                let index_y_min = y_ranges.iter().position(|r| r.contains(&min_y)).unwrap() as i64;
                let index_y_max = y_ranges.iter().position(|r| r.contains(&max_y)).unwrap() as i64;

                !(index_x_min..=index_x_max)
                    .cartesian_product(index_y_min..=index_y_max)
                    .any(|(x, y)| all_green.get(&(x, y)) == Some(&false))
            })
            .map(|([ax, ay], [bx, by])| {
                (
                    (ax.abs_diff(bx) + 1) * (ay.abs_diff(by) + 1),
                    //((ax, ay), (bx, by)),
                )
            })
            .max();
        dbg!(part2);
    }
    Ok(())
}
