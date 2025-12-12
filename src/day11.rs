use std::collections::HashMap;

use crate::graph::Graph;

pub fn part1(input: &str) {
    let (graph, name_table) = input_to_graph(input);
    let count = count_paths(
        &graph,
        *name_table.get("you").unwrap(),
        *name_table.get("out").unwrap(),
        &mut HashMap::new(),
    );
    println!("{count}");
}

pub fn part2(input: &str) {
    let (graph, name_table) = input_to_graph(input);
    let path1 = ["svr", "dac", "fft", "out"];
    let path2 = ["svr", "fft", "dac", "out"];
    let count: usize = [path1, path2]
        .iter()
        .map(|p| {
            let mut count = 1;
            for (source, sink) in p.iter().zip(p[1..].iter()) {
                count *= count_paths(
                    &graph,
                    *name_table.get(*source).unwrap(),
                    *name_table.get(*sink).unwrap(),
                    &mut HashMap::new(),
                );
            }
            count
        })
        .sum();
    println!("{count}");
}

fn name_to_i64(n: String, table: &mut HashMap<String, i64>, id_count: &mut i64) -> i64 {
    if let Some(&v) = table.get(&n) {
        v
    } else {
        table.insert(n, *id_count);
        *id_count += 1;
        *id_count - 1
    }
}

fn count_paths(g: &Graph<i64>, start: i64, end: i64, cache: &mut HashMap<i64, usize>) -> usize {
    if let Some(v) = cache.get(&start) {
        *v
    } else if start == end {
        1
    } else if let Some(l) = g.edge_list.get(&start) {
        let sol = l
            .iter()
            .map(|&(next, _)| count_paths(g, next, end, cache))
            .sum();
        cache.insert(start, sol);
        sol
    } else {
        0
    }
}

fn input_to_graph(input: &str) -> (Graph<i64>, HashMap<String, i64>) {
    let no_colon_input = input.replace(':', "");
    let mut name_table = HashMap::new();
    let mut id_count = 0;
    let mut edge_list: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    no_colon_input.trim_end().split('\n').for_each(|l| {
        let names: Vec<i64> = l
            .split_whitespace()
            .map(|n| name_to_i64(n.to_owned(), &mut name_table, &mut id_count))
            .collect();
        let k = names[0];
        let rest: Vec<(i64, i64)> = names.iter().skip(1).map(|n| (*n, 1)).collect();
        edge_list.insert(k, rest);
    });
    (Graph { edge_list }, name_table)
}
