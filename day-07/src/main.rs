use std::{collections::HashMap, mem::swap};

use itertools::Itertools;
//use regex::Regex;

#[allow(dead_code)]
fn plot(board: &[&[u8]], positions: &HashMap<(usize, usize), i64>) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if positions.contains_key(&(x, y)) {
                print!("{}", positions[&(x, y)]);
            } else {
                print!("{}", board[y][x] as char);
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

        let board = input.lines().map(|l| l.as_bytes()).collect_vec();

        let mut start = None;

        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if board[y][x] == b'S' {
                    start = Some((x, y));
                }
            }
        }
        let start = start.unwrap();

        let mut positions = HashMap::new();
        positions.insert(start, 1);
        let mut new_positions = HashMap::new();
        let mut part1 = 0i64;
        let mut part2 = 0i64;

        while !positions.is_empty() {
            //plot(&board, &positions);
            for (&(x, y), &mul) in positions.iter() {
                let (nx, ny) = (x, y + 1);
                if ny >= board.len() {
                    part2 += mul;
                    continue;
                }

                if board[ny][nx] == b'^' {
                    part1 += 1;
                    if nx > 0 && nx < board[0].len() - 1 {
                        *new_positions.entry((nx - 1, ny)).or_default() += mul;
                        *new_positions.entry((nx + 1, ny)).or_default() += mul;
                    }
                } else {
                    *new_positions.entry((nx, ny)).or_default() += mul;
                }
            }
            swap(&mut positions, &mut new_positions);
            new_positions.clear();
        }
        dbg!(part1);
        dbg!(part2);
    }
    Ok(())
}
