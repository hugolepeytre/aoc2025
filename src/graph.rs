use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct Graph<T: Copy + Hash + Eq> {
    pub edge_list: HashMap<T, Vec<(T, Cost)>>,
}

impl<T: Copy + Hash + Eq> Graph<T> {
    pub fn a_star(&self, start: T, end: T, h: &dyn Fn(T) -> Cost) -> Cost {
        // Possible to add a cameFrom map to track the actual found path
        let mut unvisited: BinaryHeap<HNode<T>> = BinaryHeap::from([HNode(start, 0)]);
        let mut distances: HashMap<T, Cost> = HashMap::from([(start, 0)]);
        while !unvisited.is_empty() {
            let HNode(current, _) = unvisited.pop().unwrap();
            if current == end {
                return distances[&current];
            }
            let curr_dist = distances[&current];
            for &(n, cost) in &self.edge_list[&current] {
                let dist = distances.entry(n).or_insert(i64::MAX);
                if curr_dist + cost < *dist {
                    *dist = curr_dist + cost;
                    // Minus because BinaryHeap is max heap. Old heuristic value will stay
                    // in the heap too because it's annoying to remove. Should not have any effect
                    unvisited.push(HNode(n, -(*dist + h(n))));
                }
            }
        }
        distances[&end]
    }

    pub fn dijkstra(&self, start: T) -> HashMap<T, Cost> {
        // Possible to add a cameFrom map to track the actual found path
        let mut unvisited: BinaryHeap<HNode<T>> = BinaryHeap::from([HNode(start, 0)]);
        let mut distances: HashMap<T, Cost> = HashMap::from([(start, 0)]);
        while !unvisited.is_empty() {
            let HNode(current, _) = unvisited.pop().unwrap();
            let curr_dist = distances[&current];
            for &(n, cost) in &self.edge_list[&current] {
                let dist = distances.entry(n).or_insert(i64::MAX);
                if curr_dist + cost < *dist {
                    *dist = curr_dist + cost;
                    // Minus because BinaryHeap is max heap. Old node value will stay
                    // in the heap too because it's annoying to remove. Should not have any effect
                    unvisited.push(HNode(n, -*dist));
                }
            }
        }
        distances
    }

    // Returns the set of nodes reachable from start
    pub fn bfs(&self, start: T) -> HashSet<T> {
        let mut visited: HashSet<T> = HashSet::new();
        let mut queue: VecDeque<T> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(curr) = queue.pop_front() {
            for &(next, _) in &self.edge_list[&curr] {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
        }
        visited
    }

    // Return each component identified by its id
    pub fn connected_components(&self) -> HashMap<usize, HashSet<T>> {
        let mut unvisited: HashSet<T> = self.edge_list.keys().copied().collect();
        let mut components: HashMap<usize, HashSet<T>> = HashMap::new();

        let mut id = 0;
        while let Some(&n) = unvisited.iter().next() {
            let component = self.bfs(n);
            let component_id = id;
            id += 1;
            unvisited = unvisited.difference(&component).copied().collect();
            components.insert(component_id, component);
        }

        components
    }

    pub fn reverse(self) -> Graph<T> {
        let mut new_edge_list: HashMap<T, Vec<(T, i64)>> = HashMap::new();
        for (node, neighbors) in self.edge_list {
            new_edge_list.entry(node).or_default();
            for (n, cost) in neighbors {
                new_edge_list.entry(n).or_default().push((node, cost));
            }
        }
        Graph {
            edge_list: new_edge_list,
        }
    }
}

#[derive(PartialEq, Eq)]
struct HNode<T>(T, Cost);

impl<T: Eq + PartialEq> Ord for HNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.1).cmp(&(other.1))
    }
}

impl<T: Eq + PartialEq> PartialOrd for HNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Cost = i64;
