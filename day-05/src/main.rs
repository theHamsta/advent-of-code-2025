use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    let number_regex = Regex::new(r"(\d+)\n").unwrap();
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let ranges = range_regex
            .captures_iter(input)
            .map(|range| {
                let min = range[1].parse::<i64>().unwrap();
                let max = range[2].parse::<i64>().unwrap();
                (min, max)
            })
            .collect_vec();
        let ids = number_regex
            .captures_iter(input.split("\n\n").nth(1).unwrap())
            .map(|range| range[1].parse::<i64>().unwrap())
            .collect_vec();

        let part1 = ids
            .iter()
            .filter(|&&id| ranges.iter().any(|&(min, max)| min <= id && id <= max))
            .count();

        let mut number_boarders = ranges
            .iter()
            .flat_map(|&(min, max)| [min, max])
            .collect_vec();
        number_boarders.sort();

        let part2: i64 = ranges
            .iter()
            .flat_map(|&(min, max)| {
                number_boarders
                    .iter()
                    .copied()
                    .tuple_windows()
                    .filter(move |(min_id, max_id)| {
                        min <= *min_id && *min_id <= max && min <= *max_id && *max_id <= max
                    })
                    .flat_map(|(min, max)| [(min, min), (min + 1, max - 1), (max, max)])
            })
            .filter(|(min, max)| min <= max)
            .unique()
            .map(|(min, max)| max - min + 1)
            .sum();

        dbg!(part1);
        dbg!(part2);
    }
    Ok(())
}
