use std::collections::HashSet;

use itertools::Itertools;
use num::Complex;

use crate::{grid::Direction, utils::parse_numbers};

pub fn part1(input: &str) {
    let count: u64 = parse_numbers::<i64>(input)
        .into_iter()
        .map(|v| (v[0], v[1]))
        .combinations(2)
        .map(|pair| {
            let [(x, y), (x2, y2)] = pair[0..2] else {
                unreachable!()
            };
            (x.abs_diff(x2) + 1) * (y.abs_diff(y2) + 1)
        })
        .max()
        .unwrap();
    println!("{count}");
}

pub fn part2(input: &str) {
    let sample = false;
    let last_step = if sample {
        Direction::Up
    } else {
        Direction::Left
    };
    let last_outside_dir = last_step.counter_clockwise();
    let coords: Vec<Complex<i32>> = parse_numbers::<i32>(input)
        .into_iter()
        .map(|v| Complex { re: v[0], im: v[1] })
        .collect();
    let mut border: HashSet<Complex<i32>> = HashSet::new();
    let mut perimeter: HashSet<Complex<i32>> = HashSet::new();

    let mut c = coords[0];
    let mut prev_step = last_step.value();
    let mut outside_dir = last_outside_dir;
    for next_c in coords.iter().skip(1).chain(coords.iter().take(1)) {
        let diff = next_c - c;
        let step = Complex {
            re: diff.re.signum(),
            im: diff.im.signum(),
        };
        outside_dir = adjust_outside_dir(outside_dir, step, prev_step);
        let mut n = c;
        border.insert(n + outside_dir.value());
        while n != *next_c {
            n += step;
            border.insert(n + outside_dir.value());
            perimeter.insert(n);
        }
        c = *next_c;
        prev_step = step;
    }

    border = border.difference(&perimeter).copied().collect();

    let best_pair = coords
        .iter()
        .combinations(2)
        .map(|pair| {
            let [&c1, &c2] = pair[0..2] else {
                unreachable!()
            };
            (
                (c1, c2),
                i64::from(c1.re.abs_diff(c2.re) + 1) * i64::from(c1.im.abs_diff(c2.im) + 1),
            )
        })
        .sorted_by_key(|p| -p.1)
        .filter(|&((c1, c2), _)| !crosses_border(c1, c2, &border))
        .take(1)
        .find_or_first(|_| false)
        .unwrap();
    let count = best_pair.1;
    println!("{count}");
}

fn crosses_border(c1: Complex<i32>, c2: Complex<i32>, border: &HashSet<Complex<i32>>) -> bool {
    let max_x = c1.re.max(c2.re);
    let max_y = c1.im.max(c2.im);
    let min_x = c1.re.min(c2.re);
    let min_y = c1.im.min(c2.im);
    for b in border {
        if min_x <= b.re && b.re <= max_x && min_y <= b.im && b.im <= max_y {
            return true;
        }
    }
    false
}

// Optimized to the point of unreadability, but I wanted to make code shorter
fn adjust_outside_dir(d: Direction, step: Complex<i32>, prev_step: Complex<i32>) -> Direction {
    let same_sign = step.im * prev_step.re + step.re * prev_step.im > 0;
    if (step.im != 0) ^ same_sign {
        d.counter_clockwise()
    } else {
        d.clockwise()
    }
}
