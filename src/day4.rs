use crate::{grid::Direction, grid::Grid, utils::string_to_grid};

pub fn part1(input: &str) {
    let mut grid = string_to_grid(input, Vec::from(['.']));
    let initial_count = grid.cells.iter().filter(|&&c| c == '@').count();
    remove_round(&mut grid);
    let final_count = grid.cells.iter().filter(|&&c| c == '@').count();
    let count = initial_count - final_count;
    println!("{count}");
}

pub fn part2(input: &str) {
    let mut grid = string_to_grid(input, Vec::from(['.']));
    let initial_count = grid.cells.iter().filter(|&&c| c == '@').count();
    while remove_round(&mut grid) {}
    let final_count = grid.cells.iter().filter(|&&c| c == '@').count();
    let count = initial_count - final_count;
    println!("{count}");
}

fn remove_round(grid: &mut Grid<char>) -> bool {
    let mut adj_counts = vec![0; grid.height * grid.width];
    let initial_count = grid.cells.iter().filter(|&&c| c == '@').count();
    for i in 0..adj_counts.len() {
        let curr_pos = grid.idx_to_pos(i);
        for d in Direction::iter() {
            if let Some(new_pos) = grid.mvt(curr_pos, d.value()) {
                if grid.get(new_pos) == '@' {
                    adj_counts[grid.pos_to_idx(curr_pos)] += 1;
                }
            }
        }
    }
    adj_counts.iter().enumerate().for_each(|(i, &c)| {
        if c < 4 && grid.get(grid.idx_to_pos(i)) == '@' {
            grid.set(grid.idx_to_pos(i), '.');
        }
    });
    let final_count = grid.cells.iter().filter(|&&c| c == '@').count();
    initial_count != final_count
}
