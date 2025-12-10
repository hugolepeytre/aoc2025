use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Direction, Grid},
    utils::string_to_grid,
};

pub fn part1(input: &str) {
    let grid = string_to_grid(input, Vec::new());
    let mut rays: Vec<num::Complex<i32>> = vec![grid.pos(&'S')];
    let mut covered = HashSet::new();
    let mut count = 0;
    while let Some(curr_pos) = rays.pop() {
        if !covered.contains(&curr_pos) {
            covered.insert(curr_pos);
            if let Some(next_pos) = grid.mvt(curr_pos, Direction::Down.value()) {
                match grid.get(next_pos) {
                    '^' => {
                        count += 1;
                        if let Some(left_split) = grid.mvt(next_pos, Direction::Left.value()) {
                            rays.push(left_split);
                        }
                        if let Some(right_split) = grid.mvt(next_pos, Direction::Right.value()) {
                            rays.push(right_split);
                        }
                    }
                    '.' => rays.push(next_pos),
                    _ => panic!("unknown grid element"),
                }
            }
        }
    }
    println!("{count}");
}

// there was a linear solution but hey I tunnelled
pub fn part2(input: &str) {
    let grid = string_to_grid(input, Vec::new());
    let mut cache = HashMap::new();
    let count = simulate_ray(&grid, &mut cache, grid.pos(&'S'));
    println!("{count}");
}

fn simulate_ray(
    grid: &Grid<char>,
    cache: &mut HashMap<num::Complex<i32>, i64>,
    pos: num::Complex<i32>,
) -> i64 {
    if let Some(&val) = cache.get(&pos) {
        val
    } else if let Some(next_pos) = grid.mvt(pos, Direction::Down.value()) {
        match grid.get(next_pos) {
            '^' => {
                let count = [
                    grid.mvt(next_pos, Direction::Left.value()),
                    grid.mvt(next_pos, Direction::Right.value()),
                ]
                .iter()
                .flatten()
                .map(|&a| simulate_ray(grid, cache, a))
                .sum();
                cache.insert(pos, count);
                count
            }
            '.' => simulate_ray(grid, cache, next_pos),
            _ => panic!("unknown grid element"),
        }
    } else {
        1
    }
}
