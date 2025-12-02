use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");

    let regex = Regex::new(r"(\d+)-(\d+)").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for range in regex.captures_iter(input) {
        let min = range[1].parse::<i64>()?;
        let max = range[2].parse::<i64>()?;

        for num in min..=max {
            let mut a = num;
            let mut digits = Vec::new();
            while a > 0 {
                digits.push(a % 10);
                a /= 10;
            }

            let (a, b) = digits.split_at(digits.len() / 2);
            if a == b {
                part1 += num;
            }

            let repeating = (1..digits.len()).find(|&length| {
                if digits.len() % length != 0 {
                    return false;
                }

                let first = &digits[..length];
                for i in 1..(digits.len() / length) {
                    let other = &digits[i * length..(i + 1) * length];
                    if other != first {
                        return false;
                    }
                }
                true
            });
            if repeating.is_some() {
                part2 += num;
            }
        }
    }

    dbg!(part1);
    dbg!(part2);
    Ok(())
}
