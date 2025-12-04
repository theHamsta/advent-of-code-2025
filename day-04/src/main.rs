use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let mut board = input
            .lines()
            .map(|l| l.trim().bytes().collect_vec())
            .collect_vec();

        let mut forklift_count = 0;
        for y in 0i64..board.len() as i64 {
            for x in 0i64..board[0].len() as i64 {
                let mut count = 0;
                let cur = board[y as usize][x as usize];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let ny = y + dy;
                        let nx = x + dx;
                        if nx < 0
                            || nx >= board[0].len() as i64
                            || ny < 0
                            || ny >= board.len() as i64
                        {
                            continue;
                        }
                        let neighbour = board[ny as usize][nx as usize];
                        if neighbour == b'@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 && cur == b'@' {
                    forklift_count += 1;
                }
            }
        }
        let part1 = forklift_count;

        let mut prev_forklift_count = None;
        let mut forklift_count = 0;
        let mut to_remove = Vec::<(i64, i64)>::new();
        loop {
            to_remove.clear();
            for y in 0i64..board.len() as i64 {
                for x in 0i64..board[0].len() as i64 {
                    let mut count = 0;
                    let cur = board[y as usize][x as usize];
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let ny = y + dy;
                            let nx = x + dx;
                            if nx < 0
                                || nx >= board[0].len() as i64
                                || ny < 0
                                || ny >= board.len() as i64
                            {
                                continue;
                            }
                            let neighbour = board[ny as usize][nx as usize];
                            if neighbour == b'@' {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 && cur == b'@' {
                        forklift_count += 1;
                        to_remove.push((x, y));
                    }
                }
            }

            for &(x, y) in to_remove.iter() {
                board[y as usize][x as usize] = b'.';
            }

            if Some(forklift_count) == prev_forklift_count {
                break;
            }
            prev_forklift_count = Some(forklift_count);
        }
        let part2 = forklift_count;

        dbg!(part1);
        dbg!(part2);
    }
    Ok(())
}
