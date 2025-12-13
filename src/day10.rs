use std::collections::HashMap;

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
    let count: usize = input
        .trim_end()
        .split('\n')
        .map(line_to_problem)
        .filter_map(|p| solve_joltage_problem(p, &mut HashMap::new()))
        .sum();
    println!("{count}");
}

fn line_to_problem(l: &str) -> Problem {
    let elems: Vec<&str> = l.split_whitespace().collect();
    let diagram = elems[0];
    let size = diagram.len() - 2;
    let mut light_target = 0;
    for c in diagram.chars() {
        if c == '.' {
            light_target <<= 1;
        }
        if c == '#' {
            light_target = (light_target << 1) + 1;
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
        .collect();
    Problem {
        light_target,
        joltage_target_evenness: evenness(&joltage_target),
        joltage_target,
        buttons,
        size,
    }
}

// General solution idea inspired by
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory
// Caching divides runtime by about 10
fn solve_joltage_problem(p: Problem, cache: &mut HashMap<Problem, Option<usize>>) -> Option<usize> {
    if p.joltage_target.iter().sum::<usize>() == 0 {
        return Some(0);
    }
    if let Some(v) = cache.get(&p) {
        return *v;
    }
    let s = p
        .buttons
        .iter()
        .powerset()
        .filter(|sol| sol.iter().fold(0, |acc, &&n| acc ^ n) == p.joltage_target_evenness)
        .filter_map(|sol| {
            if let Some(new_p) = p.push_and_halve(&sol) {
                solve_joltage_problem(new_p, cache).map(|s| (2 * s) + sol.len())
            } else {
                None
            }
        })
        .min();

    cache.insert(p, s);
    s
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Problem {
    light_target: i64,
    joltage_target: Vec<usize>,
    joltage_target_evenness: i64,
    buttons: Vec<i64>,
    size: usize,
}

impl Problem {
    fn push_and_halve(&self, pushes: &Vec<&i64>) -> Option<Problem> {
        let mut new_targets = self.joltage_target.clone();
        for b_ref in pushes {
            let mut b = **b_ref;
            let mut i = self.size;
            while b > 0 {
                i -= 1;
                if b & 1 == 1 {
                    if new_targets[i] == 0 {
                        return None;
                    }
                    new_targets[i] -= 1;
                }
                b >>= 1;
            }
        }

        let new_targets = new_targets.iter().map(|j| j / 2).collect();

        Some(Problem {
            joltage_target_evenness: evenness(&new_targets),
            joltage_target: new_targets,
            light_target: self.light_target,
            size: self.size,
            buttons: self.buttons.clone(),
        })
    }
}
fn evenness(joltages: &Vec<usize>) -> i64 {
    let mut r = 0;
    for n in joltages {
        r = (r << 1) + i64::from((n & 1) == 1);
    }
    r
}
