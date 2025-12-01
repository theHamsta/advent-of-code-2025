use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");

    let mut dial = 50;

    let regex = Regex::new(r"(R|L)(\d+)").unwrap();

    let mut part1 = 0;

    for line in regex.captures_iter(input) {
        let rotation = &line[1];
        let amount = line[2].parse::<i64>()?;

        match rotation {
            "R" => dial += amount,
            "L" => dial -= amount,
            _ => panic!(),
        }
        dial = dial.rem_euclid(100);

        if dial == 0 {
            part1 += 1;
        }
    }

    dbg!(part1);

    dial = 50;
    let mut part2 = 0;
    //dbg!(part2);
    for line in regex.captures_iter(input) {
        let rotation = &line[1];
        let mut amount = line[2].parse::<i64>()?;

        while amount > 0 {
            match rotation {
                "R" => dial += 1,
                "L" => dial -= 1,
                _ => panic!(),
            }
            dial = dial.rem_euclid(100);

            if dial == 0 {
                part2 += 1;
            }
            amount -= 1;
        }
    }
    dbg!(part2);

    Ok(())
}
