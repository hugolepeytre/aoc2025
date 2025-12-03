use crate::grid::Grid;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_to_string(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut s = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {display}: {why}");
    }
    s
}

pub fn parse_numbers_no_split<T: std::str::FromStr>(s: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();

    re.captures_iter(s.trim_end())
        .map(|c| c.extract::<0>())
        .map(|(re_match, _)| re_match.parse::<T>().unwrap())
        .collect()
}

pub fn parse_numbers<T: std::str::FromStr>(s: &str) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();

    s.trim_end()
        .split('\n')
        .map(|l| {
            re.captures_iter(l)
                .map(|c| c.extract::<0>())
                .map(|(re_match, _)| re_match.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

pub fn parse_digits<T: std::str::FromStr>(s: &str) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    let re = Regex::new(r"-?\d").unwrap();

    s.trim_end()
        .split('\n')
        .map(|l| {
            re.captures_iter(l)
                .map(|c| c.extract::<0>())
                .map(|(re_match, _)| re_match.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

pub fn parse_lists_numbers<T: std::str::FromStr>(s: Vec<&str>) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    let re = Regex::new(r"-?\d+").unwrap();

    s.into_iter()
        .map(|l| {
            re.captures_iter(l)
                .map(|c| c.extract::<0>())
                .map(|(re_match, _)| re_match.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}

pub fn string_to_grid_with_graph(input: &str, free_cells: Vec<char>) -> Grid<char> {
    let mut matrix: Vec<char> = Vec::with_capacity(input.len());
    let lines: Vec<&str> = input.trim_end().split('\n').collect();
    let height = lines.len();
    lines
        .into_iter()
        .for_each(|line| matrix.append(&mut line.chars().collect()));

    let width = matrix.len() / height;
    Grid::with_graph(matrix, width, height, free_cells)
}

pub fn string_to_grid(input: &str, free_cells: Vec<char>) -> Grid<char> {
    let mut matrix: Vec<char> = Vec::with_capacity(input.len());
    let lines: Vec<&str> = input.trim_end().split('\n').collect();
    let height = lines.len();
    lines
        .into_iter()
        .for_each(|line| matrix.append(&mut line.chars().collect()));

    let width = matrix.len() / height;
    Grid {
        cells: matrix,
        height,
        width,
        free_cells,
        edge_list: None,
    }
}
