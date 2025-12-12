use itertools::Itertools;

use crate::utils::parse_numbers_no_split;

pub fn part1(input: &str) {
    let count: usize = input
        .trim_end()
        .split('\n')
        .map(line_to_problem)
        .map(solve_light_problem)
        .sum();
    println!("{count}");
}

pub fn part2(input: &str) {
    let count = 0;
    println!("{count}");
}

fn line_to_problem(l: &str) -> Problem {
    let elems: Vec<&str> = l.split_whitespace().collect();
    let diagram = elems[0];
    let size = diagram.len() - 2;
    let mut light_target = 0;
    for c in diagram.chars() {
        if c == '.' {
            light_target *= 2;
        }
        if c == '#' {
            light_target = 2 * light_target + 1;
        }
    }
    let buttons = elems[1..(elems.len() - 1)]
        .iter()
        .map(|s| {
            let mut button = 0;
            for n in parse_numbers_no_split::<usize>(s) {
                button |= 1 << (size - 1 - n);
            }
            button
        })
        .collect();
    let joltage_target = parse_numbers_no_split::<usize>(elems[elems.len() - 1])
        .into_iter()
        .rev()
        .collect();
    Problem {
        light_target,
        joltage_target,
        buttons,
        size,
    }
}

// kinda slow but I think very elegant
fn solve_light_problem(p: Problem) -> usize {
    for sol in p.buttons.iter().powerset() {
        if sol.iter().fold(0, |acc, &&n| acc ^ n) == p.light_target {
            return sol.len();
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Problem {
    light_target: i64,
    joltage_target: Vec<usize>,
    buttons: Vec<i64>,
    size: usize,
}
