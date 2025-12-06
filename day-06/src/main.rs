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

        let numbers = input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect_vec()
            })
            .filter(|l| !l.is_empty())
            .collect_vec();

        let tasks = input
            .lines()
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .collect_vec();

        let mut total = 0i64;
        for (i, task) in tasks.iter().enumerate() {
            let mut sum = 0i64;
            let mut product = 1i64;

            for line in numbers.iter() {
                let item = line[i];
                match *task {
                    "+" => sum += item,
                    "*" => product *= item,
                    _ => panic!(),
                }
            }
            total += if *task == "+" { sum } else { product };
        }
        let part1 = total;
        dbg!(part1);

        let grid = input.lines().map(|l| l.as_bytes()).collect_vec();
        //dbg!(&grid);

        let tasks = grid
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_i, c)| !c.is_ascii_whitespace())
            .collect_vec();
        let mut part2 = 0;
        for i in 0..tasks.len() {
            let cur = tasks[i].0;
            let next = if i == tasks.len() - 1 {
                grid[0].len()
            } else {
                tasks[i + 1].0
            };

            let is_sum = *tasks[i].1 == b'+';
            let mut sum_prod = if is_sum { 0i64 } else { 1 };
            for col in (cur..next).rev() {
                let num = (0..(grid.len() - 1))
                    .map(|row| grid[row][col])
                    .filter(|&c| (c as char).is_ascii_digit())
                    .collect_vec();
                if num.is_empty() {
                    continue;
                }
                let num_str = String::from_utf8(num).unwrap();
                let num = num_str.parse::<i64>().unwrap();
                if is_sum {
                    sum_prod += num;
                } else {
                    sum_prod *= num;
                }
                //dbg!(num_str, sum_prod, *tasks[i].1 as char, is_sum);
            }
            part2 += sum_prod;
        }
        dbg!(part2);
    }
    Ok(())
}
