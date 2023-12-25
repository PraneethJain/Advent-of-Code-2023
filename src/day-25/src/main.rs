use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> usize {
    let mut nodes: BTreeSet<&str> = BTreeSet::new();
    let mut edges: BTreeSet<(&str, &str)> = BTreeSet::new();
    for line in lines.lines() {
        match line.split_once(':') {
            Some((l, r)) => r.split_whitespace().for_each(|x| {
                nodes.insert(l);
                nodes.insert(x);
                edges.insert((l, x));
            }),
            None => panic!("no : in line {}", line),
        }
    }

    let node_map: BTreeMap<&str, u32> = nodes
        .iter()
        .enumerate()
        .map(|(a, &b)| (b, a as u32))
        .collect();
    let edge_vec: Vec<(u32, u32)> = edges
        .iter()
        .map(|&x| (node_map[x.0], node_map[x.1]))
        .collect();

    let graph: UnGraph<(), ()> = UnGraph::from_edges(edge_vec);

    let min_cut_res: Result<Option<(usize, Vec<NodeIndex>)>, ()> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_, partition) = min_cut_res.unwrap().unwrap();

    partition.len() * (graph.node_count() - partition.len())
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();

    if let Err(why) = file.read_to_string(&mut lines) {
        panic!("couldn't read {}: {}", display, why)
    }

    println!("{}", part_one(&lines));
}
