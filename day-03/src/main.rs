use itertools::Itertools;
//use regex::Regex;

fn maximize(bank: &[u8], n: u64) -> Vec<(u64, u64, Vec<usize>)> {
    if n == 0 {
        return vec![(0, 0, vec![])];
    }

    let indices = maximize(bank, n - 1);

    let best_choices = indices
        .iter()
        .cartesian_product(0..bank.len())
        .filter(|((_, _, indices), i)| !indices.contains(i))
        .map(|((_, _, indices), i)| {
            let mut total = 0u64;
            let mut value = 1u64;
            let mut counted = false;
            for &n in indices.iter().rev() {
                if i > n && !counted {
                    total += (bank[i] - b'0') as u64 * value;
                    value *= 10;
                    counted = true;
                }
                total += (bank[n] - b'0') as u64 * value;
                value *= 10;
            }
            if !counted {
                total += (bank[i] - b'0') as u64 * value;
            }
            (i, indices, total)
        })
        .max_set_by_key(|a| a.2);

    best_choices
        .iter()
        .map(|&(new_index, indices, score)| {
            let mut clone = indices.clone();
            clone.push(new_index);
            clone.sort();

            let mut number = 0u64;
            for &c in clone.iter() {
                number *= 10;
                number += (bank[c] - b'0') as u64;
            }
            (score, number, clone)
        })
        .unique_by(|a| a.1)
        .collect_vec()
}

fn main() -> anyhow::Result<()> {
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);
        let mut part1 = 0;
        let mut part2 = 0;

        for bank in input.lines() {
            let bank = bank.trim();

            let max = (0..bank.len())
                .tuple_combinations()
                .flat_map(|(i, j)| {
                    let a = (bank.as_bytes()[i] - b'0') as u64;
                    let b = (bank.as_bytes()[j] - b'0') as u64;
                    (i < j).then_some(a * 10 + b)
                })
                .max()
                .unwrap();
            part1 += max;

            //dbg!(bank);
            let tmp = maximize(bank.as_bytes(), 12);
            let result = tmp.first().unwrap();
            //dbg!(result);

            part2 += result.0;
        }

        dbg!(part1);
        dbg!(part2);
    }
    Ok(())
}
