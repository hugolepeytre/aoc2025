use core::panic;

use itertools::Itertools;
use num::complex::Complex;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Grid<T: Copy + Eq> {
    pub cells: Vec<T>,
    pub width: usize,
    pub height: usize,
    pub free_cells: Vec<T>,
    pub edge_list: Option<Vec<Vec<usize>>>,
}

impl<T: Copy + Eq> Grid<T> {
    pub fn with_graph(grid: Vec<T>, width: usize, height: usize, free_cells: Vec<T>) -> Grid<T> {
        let mut grid = Grid {
            cells: grid,
            width,
            height,
            free_cells,
            edge_list: None,
        };
        grid.make_graph();
        grid
    }

    pub fn mvt(&self, pos: Complex<i32>, mvt: Complex<i32>) -> Option<Complex<i32>> {
        let new_pos = pos + mvt;
        if self.inbounds(new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn get(&self, pos: Complex<i32>) -> T {
        self.cells[self.pos_to_idx(pos)]
    }

    pub fn pos(&self, e: &T) -> Complex<i32> {
        let pos = self.cells.iter().position(|elem| elem == e).unwrap();
        self.idx_to_pos(pos)
    }

    pub fn set(&mut self, pos: Complex<i32>, e: T) {
        let idx = self.pos_to_idx(pos);
        self.cells[idx] = e;
    }

    pub fn inbounds(&self, pos: Complex<i32>) -> bool {
        0 <= pos.re
            && pos.re < i32::try_from(self.width).unwrap()
            && 0 <= pos.im
            && pos.im < i32::try_from(self.height).unwrap()
    }

    pub fn pos_to_idx(&self, pos: Complex<i32>) -> usize {
        pos.re as usize + pos.im as usize * self.width
    }

    pub fn idx_to_pos(&self, idx: usize) -> Complex<i32> {
        Complex {
            re: (idx % self.width) as i32,
            im: (idx / self.width) as i32,
        }
    }

    pub fn make_graph(&mut self) {
        let mut edge_list = Vec::new();
        for (idx, c) in self.cells.iter().enumerate() {
            let mut edges = Vec::new();
            if self.free_cells.contains(c) {
                let pos = self.idx_to_pos(idx);
                for n in Direction::iter_straight()
                    .into_iter()
                    .filter_map(|d| self.mvt(pos, d.value()))
                {
                    if self.free_cells.contains(&self.get(n)) {
                        edges.push(self.pos_to_idx(n));
                    }
                }
            }
            edge_list.push(edges);
        }
        self.edge_list = Some(edge_list);
    }

    // Returns the set of nodes reachable from start
    pub fn bfs(&self, start: Complex<i32>) -> HashSet<Complex<i32>> {
        let edge_list = self.edge_list();
        let start = self.pos_to_idx(start);
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(curr) = queue.pop_front() {
            for &next in &edge_list[curr] {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
        }
        visited.into_iter().map(|i| self.idx_to_pos(i)).collect()
    }

    // BFS but storing distances
    pub fn dijkstra(&self, start: Complex<i32>) -> HashMap<Complex<i32>, usize> {
        let edge_list = self.edge_list();
        let start = self.pos_to_idx(start);
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        distances.insert(start, 0);
        while let Some(curr) = queue.pop_front() {
            for &next in &edge_list[curr] {
                if !visited.contains(&next) {
                    visited.insert(next);
                    distances.insert(next, distances[&curr] + 1);
                    queue.push_back(next);
                }
            }
        }
        distances
            .into_iter()
            .map(|(k, v)| (self.idx_to_pos(k), v))
            .collect()
    }

    // Return each component identified by its id
    pub fn connected_components(&self) -> HashMap<usize, HashSet<Complex<i32>>> {
        let edge_list = self.edge_list();
        let mut unvisited: HashSet<Complex<i32>> =
            (0..edge_list.len()).map(|i| self.idx_to_pos(i)).collect();
        let mut components: HashMap<usize, HashSet<Complex<i32>>> = HashMap::new();

        while let Some(&n) = unvisited.iter().next() {
            let component = self.bfs(n);
            let component_id = self.pos_to_idx(*component.iter().next().unwrap());
            unvisited = unvisited.difference(&component).copied().collect();
            components.insert(component_id, component);
        }

        components
    }

    pub fn count_paths(&self, start: usize, target: &T) -> usize {
        let edge_list = self.edge_list();
        let mut sum = 0;
        for &next in &edge_list[start] {
            if target == &self.cells[next] {
                sum += 1;
            } else {
                sum += self.count_paths(next, target);
            }
        }
        sum
    }

    pub fn n_neighbors(&self, i: Complex<i32>) -> usize {
        self.edge_list()[self.pos_to_idx(i)].len()
    }

    pub fn edge_list(&self) -> &Vec<Vec<usize>> {
        assert!(self.edge_list.is_some(), "Graph not initialized on grid");
        self.edge_list.as_ref().unwrap()
    }
}

impl Grid<char> {
    pub fn _print(&self) {
        let s = (0..self.height)
            .map(|i| {
                let begin = i * self.width;
                let s: String = self.cells[begin..begin + self.width].iter().collect();
                s
            })
            .join("\n");
        println!("{s}");
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn value(self) -> Complex<i32> {
        match self {
            Direction::Up => Complex { im: -1, re: 0 },
            Direction::Down => Complex { im: 1, re: 0 },
            Direction::Left => Complex { im: 0, re: -1 },
            Direction::Right => Complex { im: 0, re: 1 },
            Direction::UpLeft => Complex { im: -1, re: -1 },
            Direction::UpRight => Complex { im: -1, re: 1 },
            Direction::DownLeft => Complex { im: 1, re: -1 },
            Direction::DownRight => Complex { im: 1, re: 1 },
        }
    }

    pub fn from_value(value: Complex<i32>) -> Direction {
        match value {
            Complex { im: -1, re: 0 } => Direction::Up,
            Complex { im: 1, re: 0 } => Direction::Down,
            Complex { im: 0, re: -1 } => Direction::Left,
            Complex { im: 0, re: 1 } => Direction::Right,
            Complex { im: -1, re: -1 } => Direction::UpLeft,
            Complex { im: -1, re: 1 } => Direction::UpRight,
            Complex { im: 1, re: -1 } => Direction::DownLeft,
            Complex { im: 1, re: 1 } => Direction::DownRight,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn counter_clockwise(self) -> Direction {
        let v = self.value() * Complex { im: -1, re: 0 };
        Direction::from_value(v)
    }

    pub fn clockwise(self) -> Direction {
        let v = self.value() * Complex { im: 1, re: 0 };
        Direction::from_value(v)
    }

    pub fn opposite(self) -> Direction {
        let v = self.value() * Complex { im: 0, re: -1 };
        Direction::from_value(v)
    }

    pub fn from_arrow(a: char) -> Direction {
        match a {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            c => panic!("Not an arrow: {}", c),
        }
    }

    pub fn iter_diags() -> [Direction; 4] {
        [
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }

    pub fn iter_straight() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn iter() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}
