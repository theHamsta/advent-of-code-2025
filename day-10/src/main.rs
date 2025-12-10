use good_lp::{Expression, coin_cbc, variable};
use good_lp::{Solution, SolverModel, constraint, default_solver, variables};
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

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
    let number_regex = Regex::new(r"(\d+)").unwrap();
    for (file, input) in [
        ("input", include_str!("../input")),
        ("example", include_str!("../example")),
    ] {
        dbg!(file);

        let numbers = input
            .lines()
            .map(|l| {
                let mut it = l.split(" ");
                let indicator = it.next().unwrap();
                let indicator = &indicator[1..indicator.len() - 1];
                let mut numbers = Vec::new();
                for a in it {
                    let n = number_regex
                        .captures_iter(a)
                        .flat_map(|c| c[0].parse::<i64>())
                        .collect_vec();
                    numbers.push(n);
                }
                let goal = indicator.chars().enumerate().fold(0i64, |acc, (i, c)| {
                    acc + match c {
                        '#' => 1 << i,
                        '.' => 0,
                        _ => panic!(),
                    }
                });

                (goal, numbers)
            })
            .collect_vec();

        let mut part1 = 0i64;
        for (goal, switches) in numbers.iter() {
            let allowed_switches = &switches[0..switches.len() - 1];
            let transitions = allowed_switches
                .iter()
                .map(|n| n.iter().fold(0i64, |acc, &bit| acc | (1 << bit)))
                .collect_vec();
            //let mut shortest = HashMap::new();
            let mut visited = HashSet::new();
            //dbg!(goal, &transitions);

            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), 0));

            let mut found = false;
            while let Some((Reverse(cost), state)) = queue.pop() {
                if visited.contains(&state) {
                    continue;
                }
                visited.insert(state);
                if state == *goal {
                    //dbg!(cost, state);
                    part1 += cost;
                    found = true;
                    break;
                }

                for t in transitions.iter().copied() {
                    let next = state ^ t;
                    //dbg!(next);
                    queue.push((Reverse(cost + 1), next));
                }
            }
            //dbg!(visited);
            assert!(found);
        }
        dbg!(part1);

        let mut part2 = 0i64;
        for (_goal, switches) in numbers.iter() {
            let new_goal = &switches.last().unwrap()[..];
            let allowed_switches = &switches[0..switches.len() - 1];

            let mut vars = variables!();
            let var_vec = (0..allowed_switches.len())
                .map(|_| vars.add(variable().integer().min(0)))
                .collect_vec();

            let mut solution = vars
                .minimise::<Expression>(var_vec.iter().sum())
                .using(good_lp::default_solver);

            for (i, g) in new_goal.iter().enumerate() {
                solution = solution.with(
                    var_vec
                        .iter()
                        .enumerate()
                        .filter(|&(j, _v)| allowed_switches[j].contains(&(i as i64)))
                        .map(|(_, v)| v)
                        .sum::<Expression>()
                        .eq(Expression::from_other_affine(*g as f64)),
                );
            }

            let solved = solution.solve().unwrap();
            let sum = solved.eval(var_vec.iter().sum::<Expression>());

            part2 += sum as i64;

            //dbg!(visited);
        }
        dbg!(part2);
    }
    Ok(())
}
