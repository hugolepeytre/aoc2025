use crate::utils::parse_numbers;

pub fn part1(input: &str) {
    let parsed_integers: Vec<i32> = parse_numbers::<i32>(&input.replace('L', "-"))
        .iter()
        .flatten()
        .copied()
        .collect();
    let (z_count, final_value) = parsed_integers
        .iter()
        .fold((0, 50), |(z_count, val), instr| {
            (z_count + i32::from(val == 0), (val + instr).rem_euclid(100))
        });
    let z_count = z_count + i32::from(final_value == 0);
    println!("{z_count}");
}

pub fn part2(input: &str) {
    let parsed_integers: Vec<i32> = parse_numbers::<i32>(&input.replace('L', "-"))
        .iter()
        .flatten()
        .copied()
        .collect();
    let (z_count, _) = parsed_integers
        .iter()
        .fold((0, 50), |(z_count, prev_click_val), &instr| {
            let new_val = (prev_click_val + instr).rem_euclid(100);
            let full_rots = num::Integer::div_floor(&instr.abs(), &100);
            let zero_clicked = ((instr < 0 && new_val > prev_click_val)
                || (instr > 0 && new_val < prev_click_val)
                || new_val == 0)
                && prev_click_val != 0;
            (z_count + i32::from(zero_clicked) + full_rots, new_val)
        });
    println!("{z_count}");
}
