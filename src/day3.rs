use crate::utils::parse_digits;

pub fn part1(input: &str) {
    let parsed_digits: Vec<Vec<i64>> = parse_digits::<i64>(input);
    let count: i64 = parsed_digits
        .iter()
        .filter_map(|v| find_max_battery_val(v, 2))
        .sum();
    println!("{count}");
}

fn find_max_battery_val(v: &[i64], n: u32) -> Option<i64> {
    if n == 1 {
        v.iter().max().copied()
    } else {
        let max_first_digit = v.iter().take(v.len() + 1 - n as usize).max()?;
        let idx_max_first_digit = v.iter().position(|n| n == max_first_digit)?;
        let max_remaining_val = find_max_battery_val(&v[(idx_max_first_digit + 1)..], n - 1)?;
        Some(max_first_digit * 10_i64.pow(n - 1) + max_remaining_val)
    }
}

pub fn part2(input: &str) {
    let parsed_digits: Vec<Vec<i64>> = parse_digits::<i64>(input);
    let count: i64 = parsed_digits
        .iter()
        .filter_map(|v| find_max_battery_val(v, 12))
        .sum();
    println!("{count}");
}
