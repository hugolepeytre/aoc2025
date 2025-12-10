use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{graph::Graph, utils::parse_numbers};

pub fn part1(input: &str) {
    let first_n_edges = 1000;
    let first_n_ccs = 3;
    let boxes: Vec<_> = parse_numbers::<i64>(input)
        .into_iter()
        .map(|v| {
            if let &[x, y, z] = &v[..] {
                (x, y, z)
            } else {
                panic!("didn't find 3 coordinates")
            }
        })
        .collect();
    let edges: Vec<(Coord, Coord, i64)> = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1, z1))| {
            boxes
                .iter()
                .skip(i + 1)
                .filter_map(|&(x2, y2, z2)| {
                    if x1 == x2 && y1 == y2 && z1 == z2 {
                        None
                    } else {
                        let dist = ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt();
                        Some(((x1, y1, z1), (x2, y2, z2), dist))
                    }
                })
                .collect::<Vec<(Coord, Coord, i64)>>()
        })
        .sorted_by_key(|(_, _, d)| *d)
        .take(first_n_edges)
        .collect();
    let mut graph_edges: HashMap<Coord, Vec<(Coord, i64)>> = HashMap::new();
    for (c1, c2, cost) in edges {
        let entry = graph_edges.entry(c1).or_default();
        entry.push((c2, cost));
        let entry = graph_edges.entry(c2).or_default();
        entry.push((c1, cost));
    }
    let graph = Graph {
        edge_list: graph_edges,
    };
    println!("yea");
    let count: usize = graph
        .connected_components()
        .into_values()
        .map(|cc| cc.len())
        .sorted()
        .rev()
        .take(first_n_ccs)
        .product();
    println!("{count}");
}

// there was a linear solution but hey I tunnelled
pub fn part2(input: &str) {
    let boxes: Vec<_> = parse_numbers::<i64>(input)
        .into_iter()
        .map(|v| {
            if let &[x, y, z] = &v[..] {
                (x, y, z)
            } else {
                panic!("didn't find 3 coordinates")
            }
        })
        .collect();
    let mut edges = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, &(x1, y1, z1))| {
            boxes
                .iter()
                .skip(i + 1)
                .filter_map(|&(x2, y2, z2)| {
                    if x1 == x2 && y1 == y2 && z1 == z2 {
                        None
                    } else {
                        let dist = ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt();
                        Some(((x1, y1, z1), (x2, y2, z2), dist))
                    }
                })
                .collect::<Vec<(Coord, Coord, i64)>>()
        })
        .sorted_by_key(|(_, _, d)| *d);

    let mut circuit_count = 0;
    let mut circuits: HashMap<usize, HashSet<Coord>> = HashMap::new();
    let mut coord_to_circuit: HashMap<Coord, usize> = HashMap::new();

    circuits.insert(circuit_count, HashSet::from_iter([boxes[0]]));
    coord_to_circuit.insert(boxes[0], 0);
    circuit_count += 1;

    let mut next_pair: (Coord, Coord, i64) = ((0, 0, 0), (0, 0, 0), 0);
    while circuits.len() > 1 || circuits.values().next().unwrap().len() < boxes.len() {
        next_pair = edges.next().unwrap();
        let (c1, c2, _) = next_pair;
        if coord_to_circuit.contains_key(&c1) {
            if coord_to_circuit.contains_key(&c2) {
                let s1 = coord_to_circuit[&c1];
                let s2 = coord_to_circuit[&c2];
                if s1 != s2 {
                    let circ2 = circuits.remove(&s2).unwrap();
                    if let Some(circ1) = circuits.get_mut(&s1) {
                        for coord in circ2 {
                            coord_to_circuit.insert(coord, s1);
                            circ1.insert(coord);
                        }
                    }
                }
            } else {
                let s = coord_to_circuit[&c1];
                coord_to_circuit.insert(c2, s);
                circuits.entry(s).and_modify(|c| {
                    c.insert(c2);
                });
            }
        } else if coord_to_circuit.contains_key(&c2) {
            let s = coord_to_circuit[&c2];
            coord_to_circuit.insert(c1, s);
            circuits.entry(s).and_modify(|c| {
                c.insert(c1);
            });
        } else {
            circuits.insert(circuit_count, HashSet::from_iter([c1, c2]));
            coord_to_circuit.insert(c1, circuit_count);
            coord_to_circuit.insert(c2, circuit_count);
            circuit_count += 1;
        }
    }
    let count = next_pair.0 .0 * next_pair.1 .0;
    println!("{count}");
}

type Coord = (i64, i64, i64);
