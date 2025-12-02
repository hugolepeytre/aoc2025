use std::collections::HashSet;

use crate::utils::parse_numbers;

pub fn part1(input: &str) {
    let parsed_integers: Vec<i64> = parse_numbers::<i64>(&input.replace('-', "x"))
        .iter()
        .flatten()
        .copied()
        .collect();
    let ranges: Vec<(i64, i64)> = parsed_integers
        .iter()
        .step_by(2)
        .zip(parsed_integers.iter().skip(1).step_by(2))
        .map(|(&a, &b)| (a, b))
        .collect();
    let max_val = *parsed_integers.iter().max().unwrap_or(&0);
    dbg!(&ranges);
    let mut count = 0;
    let mut idx: i64 = 1;
    let idx_digits = idx.checked_ilog10().unwrap_or(0) + 1;
    let mut tested_val = idx + (idx * 10_i64.pow(idx_digits));
    while tested_val < max_val {
        if ranges
            .iter()
            .any(|(low, high)| *low <= tested_val && tested_val <= *high)
        {
            count += tested_val;
        }
        idx += 1;
        let idx_digits = idx.checked_ilog10().unwrap_or(0) + 1;
        tested_val = idx + (idx * 10_i64.pow(idx_digits));
    }
    println!("{count}");
}

pub fn part2(input: &str) {
    let mut invalid_ids: HashSet<i64> = HashSet::new();
    let parsed_integers: Vec<i64> = parse_numbers::<i64>(&input.replace('-', "x"))
        .iter()
        .flatten()
        .copied()
        .collect();
    let ranges: Vec<(i64, i64)> = parsed_integers
        .iter()
        .step_by(2)
        .zip(parsed_integers.iter().skip(1).step_by(2))
        .map(|(&a, &b)| (a, b))
        .collect();
    let max_val = *parsed_integers.iter().max().unwrap_or(&0);
    dbg!(max_val);
    dbg!(self_concat_number(4, 2));
    let mut idx: i64 = 1;
    while self_concat_number(idx, 2) < max_val {
        let mut n_concat = 2;
        while self_concat_number(idx, n_concat) < max_val {
            let tested_id = self_concat_number(idx, n_concat);
            if ranges
                .iter()
                .any(|(low, high)| *low <= tested_id && tested_id <= *high)
            {
                invalid_ids.insert(tested_id);
            }
            n_concat += 1;
        }
        idx += 1;
    }
    let count: i64 = invalid_ids.iter().sum();
    println!("{count}");
}

fn self_concat_number(num: i64, n: u32) -> i64 {
    let n_digits = num.checked_ilog10().unwrap_or(0) + 1;
    (0..n).map(|i| num * 10_i64.pow(i * n_digits)).sum()
}
