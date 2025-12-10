use std::collections::HashSet;

use itertools::Itertools;
use num::{Complex, Signed};

use crate::{
    grid::{Direction, Grid},
    utils::parse_numbers,
};

pub fn part1(input: &str) {
    let coords: Vec<(i64, i64)> = parse_numbers::<i64>(input)
        .into_iter()
        .map(|v| (v[0], v[1]))
        .collect();
    let count: i64 = coords
        .iter()
        .enumerate()
        .flat_map(|(i, &(x, y))| {
            coords
                .iter()
                .skip(i + 1)
                .map(|(x2, y2)| ((x.abs_sub(x2) + 1) * (y.abs_sub(y2) + 1)))
                .collect::<Vec<_>>()
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
    let last_outside_dir = if sample {
        last_step.counter_clockwise()
    } else {
        last_step.counter_clockwise() // try both
    };
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
    visu(&border, Complex { im: 0, re: 0 }, Complex { im: 0, re: 0 });
    // let sol1 = Complex {
    //     re: 5953,
    //     im: 50262,
    // };
    // let sol2 = Complex {
    //     re: 94872,
    //     im: 67629,
    // };

    let mut i = 0;
    let best_pair = coords
        .iter()
        .enumerate()
        .flat_map(|(i, &c1)| {
            coords
                .iter()
                .copied()
                .skip(i + 1)
                .map(|c2| {
                    (
                        (c1, c2),
                        i64::from(c1.re.abs_diff(c2.re) + 1) * i64::from(c1.im.abs_diff(c2.im) + 1),
                    )
                })
                .collect::<Vec<_>>()
        })
        .sorted_by_key(|p| -p.1)
        .filter(|&((c1, c2), _)| {
            println!("{i}/{}", coords.len() * (coords.len() + 1) / 2);
            i += 1;
            !crosses_border(c1, c2, &border)
        })
        .take(1)
        .find_or_first(|_| false)
        .unwrap();
    let count = best_pair.1;
    println!("{count}");
}

fn crosses_border(c1: Complex<i32>, c2: Complex<i32>, border: &HashSet<Complex<i32>>) -> bool {
    let perimeter: HashSet<Complex<i32>> = get_increasing_range_exclusive(c1.re, c2.re)
        .flat_map(|i| [Complex { re: i, im: c1.im }, Complex { re: i, im: c2.im }])
        .chain(
            get_increasing_range_exclusive(c1.im, c2.im)
                .flat_map(|j| [Complex { re: c1.re, im: j }, Complex { re: c2.re, im: j }]),
        )
        .collect();
    let inter = perimeter.iter().any(|c| border.contains(c));
    inter
}

fn adjust_outside_dir(d: Direction, step: Complex<i32>, prev_step: Complex<i32>) -> Direction {
    match step {
        Complex { im: -1, re: 0 } => {
            if prev_step.re == 1 {
                d.counter_clockwise()
            } else {
                d.clockwise()
            }
        }
        Complex { im: 1, re: 0 } => {
            if prev_step.re == 1 {
                d.clockwise()
            } else {
                d.counter_clockwise()
            }
        }
        Complex { im: 0, re: -1 } => {
            if prev_step.im == 1 {
                d.clockwise()
            } else {
                d.counter_clockwise()
            }
        }
        Complex { im: 0, re: 1 } => {
            if prev_step.im == 1 {
                d.counter_clockwise()
            } else {
                d.clockwise()
            }
        }
        _ => panic!("Invalid direction"),
    }
}

fn get_increasing_range_exclusive(a: i32, b: i32) -> std::ops::Range<i32> {
    if a < b {
        a + 1..b
    } else {
        b + 1..a
    }
}

fn visu(border: &HashSet<Complex<i32>>, a: Complex<i32>, b: Complex<i32>) {
    fn divide_1000(c: Complex<i32>) -> Complex<i32> {
        Complex {
            re: c.re / 1000,
            im: c.im / 1000,
        }
    }
    let mut g = Grid {
        cells: vec!['.'; 100 * 100],
        width: 100,
        height: 100,
        free_cells: Vec::new(),
        edge_list: None,
    };
    for b in border {
        g.set(divide_1000(*b), '@');
    }
    g.set(divide_1000(a), '#');
    g.set(divide_1000(b), '#');
    g.set(
        divide_1000(Complex {
            re: 5953,
            im: 50262,
        }),
        '?',
    );
    g.set(
        divide_1000(Complex {
            re: 94872,
            im: 67629,
        }),
        '?',
    );
    g._print();
}
