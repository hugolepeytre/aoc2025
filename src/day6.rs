use crate::utils::parse_numbers;
use itertools::izip;

pub fn part1(input: &str) {
    let (input_numbers, input_ops) = input.trim_end().rsplit_once('\n').unwrap();
    let lists = parse_numbers::<i64>(input_numbers);
    let ops: Vec<&str> = input_ops.split_whitespace().collect();
    let (l1, l2, l3, l4) = (
        lists[0].clone(),
        lists[1].clone(),
        lists[2].clone(),
        lists[3].clone(),
    );
    let count: i64 = izip!(l1, l2, l3, l4, ops)
        .map(|(n1, n2, n3, n4, op)| {
            dbg!(n1, n2, n3, n4);
            match op {
                "*" => n1 * n2 * n3 * n4,
                "+" => n1 + n2 + n3 + n4,
                _ => panic!("unknown op"),
            }
        })
        .sum();
    println!("{count}");
}

pub fn part2(input: &str) {
    let chars: Vec<Vec<char>> = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let (total_sum, last_prob_sum, _) = izip!(
        chars[0].clone(),
        chars[1].clone(),
        chars[2].clone(),
        chars[3].clone(),
        chars[4].clone()
    )
    .fold(
        (0, 0, '+'),
        |(total_sum, mut prob_sum, mut curr_op), (n1, n2, n3, n4, op)| {
            let mut val = 0;
            if op != ' ' {
                curr_op = op;
                prob_sum = match op {
                    '+' => 0,
                    '*' => 1,
                    _ => panic!("unknown op"),
                }
            }
            for n in [n1, n2, n3, n4] {
                if let Some(v) = n.to_digit(10).map(i64::from) {
                    val = 10 * val + v;
                }
            }
            if val == 0 {
                println!("adding {prob_sum}");
                (total_sum + prob_sum, 0, 'x')
            } else {
                println!("opping {val}");
                let new_prob_sum = match curr_op {
                    '+' => prob_sum + val,
                    '*' => prob_sum * val,
                    _ => panic!("unknown op"),
                };
                (total_sum, new_prob_sum, curr_op)
            }
        },
    );
    let count = total_sum + last_prob_sum;
    println!("{count}");
}
