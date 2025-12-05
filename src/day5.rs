use crate::utils::{parse_numbers, parse_numbers_no_split};

// Switching from hashset to breaking in the if only divided runtime by 2
pub fn part1(input: &str) {
    let input = input.replace('-', ";");
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(i64, i64)> = parse_numbers::<i64>(ranges_str)
        .iter()
        .map(|v| {
            if let [r1, r2] = v[..2] {
                (r1, r2)
            } else {
                (0, 0)
            }
        })
        .collect();
    let ids = parse_numbers_no_split::<i64>(ids_str);
    let mut count = 0;
    for id in ids {
        for &(low, high) in &ranges {
            if low <= id && id <= high {
                count += 1;
                break;
            }
        }
    }
    println!("{count}");
}

// To make this more efficient, sort ranges first then merge is O(n) instead of O(n^2)
pub fn part2(input: &str) {
    let input = input.replace('-', ";");
    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(i64, i64)> = parse_numbers::<i64>(ranges_str)
        .iter()
        .map(|v| {
            if let [r1, r2] = v[..2] {
                (r1, r2)
            } else {
                (0, 0)
            }
        })
        .collect();
    let mut final_ranges: Vec<(i64, i64)> = Vec::new();
    while ranges.len() > 1 {
        let mut candidate = ranges.pop().unwrap();
        let mut mergers = vec![candidate];
        while !mergers.is_empty() {
            let (no_overlap, overlap): (Vec<_>, Vec<_>) = ranges
                .iter()
                .partition(|&&(low, high)| high < candidate.0 || low > candidate.1);
            mergers = overlap;
            ranges = no_overlap;
            let min_low = mergers
                .iter()
                .map(|&(low, _)| low)
                .min()
                .unwrap_or(candidate.0);
            let max_high = mergers
                .iter()
                .map(|&(_, high)| high)
                .max()
                .unwrap_or(candidate.1);
            let min_low = min_low.min(candidate.0);
            let max_high = max_high.max(candidate.1);
            candidate = (min_low, max_high);
        }
        final_ranges.push(candidate);
    }
    if !ranges.is_empty() {
        final_ranges.push(ranges[0]);
    }
    let count: i64 = final_ranges.iter().map(|&(low, high)| high - low + 1).sum();
    println!("{count}");
}
