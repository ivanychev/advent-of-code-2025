use crate::args::Args;
use crate::utils::input::read_input_lines;
use roaring::RoaringBitmap;
use std::collections::HashMap;
use topo_sort::{SortResults, TopoSort};

struct Graph {
    adjacency_list: Vec<Vec<u32>>,
    name_to_idx: HashMap<String, u32>,
    node_names: Vec<String>,
}

impl Graph {
    fn get_name_by_idx(&self, idx: u32) -> &str {
        &self.node_names[idx as usize]
    }
}

struct DfsState {
    current_node: u32,
    visited_list_idx: Option<u32>,
}

impl DfsState {
    fn with_visited_idx(&self, idx: u32) -> Self {
        DfsState {
            current_node: self.current_node,
            visited_list_idx: Some(idx),
        }
    }
}

fn edges_to_topsorted<'a>(edges: &'a Vec<(&'a str, Vec<&'a str>)>) -> Vec<&'a str> {
    let mut topo_sort = TopoSort::with_capacity(edges.len());
    for (node, connections) in edges.iter() {
        topo_sort.insert(*node, connections.clone())
    }
    match topo_sort.into_vec_nodes() {
        SortResults::Full(nodes) => nodes.into_iter().rev().collect(),
        _ => {
            panic!("Failed to topologically sort the graph - it may contain cycles");
        }
    }
}

impl Graph {
    fn from_edges(edges: Vec<String>) -> Self {
        // Example
        // aaa: you hhh
        let raw_node_to_nodes = {
            let mut vec: Vec<(&str, Vec<&str>)> = edges
                .iter()
                .map(|line| {
                    let (node_name, connected_to_raw) = line.split_once(':').unwrap();
                    let connections: Vec<&str> = connected_to_raw.split_whitespace().collect();
                    (node_name.trim(), connections)
                })
                .collect();
            vec.push(("out", vec![])); // 'out' node always exists but not in the input
            vec
        };

        let top_sorted_nodes = edges_to_topsorted(&raw_node_to_nodes);

        let node_to_idx: HashMap<&str, u32> = top_sorted_nodes
            .iter()
            .enumerate()
            .map(|(idx, &name)| (name, idx as u32))
            .collect();

        let mut adjacency_list: Vec<Vec<u32>> = vec![Vec::new(); top_sorted_nodes.len()];

        for (node_name, connections) in raw_node_to_nodes.iter() {
            let node_idx = *node_to_idx.get(node_name).unwrap() as usize;
            let adjacency = &mut adjacency_list[node_idx];
            for conn in connections.iter() {
                let conn_idx = *node_to_idx.get(conn).unwrap();
                adjacency.push(conn_idx);
            }
        }

        Graph {
            adjacency_list,
            name_to_idx: node_to_idx
                .iter()
                .map(|(&name, &idx)| (name.to_string(), idx))
                .collect(),
            node_names: top_sorted_nodes
                .iter()
                .map(|&name| name.to_string())
                .collect(),
        }
    }

    fn scan_paths(&self, start_node: &str, end_node: &str) -> usize {
        let start_node_idx = *self.name_to_idx.get(start_node).unwrap();
        let end_node_idx = *self.name_to_idx.get(end_node).unwrap();
        if start_node_idx > end_node_idx {
            // No paths possible in a DAG
            return 0;
        }
        let mut stack: Vec<DfsState> = vec![DfsState {
            current_node: start_node_idx,
            visited_list_idx: None,
        }];
        let mut visited = RoaringBitmap::from([start_node_idx]);
        let mut total_paths = 0usize;
        while let Some(state) = stack.pop() {
            visited.remove(state.current_node);
            // if state.visited_list_idx.is_none() {
            //     // Just entered this node
            // }
            // println!("Entered node {}, visited: {:?}", self.node_names[state.current_node as usize], visited);

            if state.current_node == end_node_idx {
                // Found a path to the end
                total_paths += 1;
                continue;
            }

            let next_to_visit_start = state.visited_list_idx.map(|idx| idx + 1).unwrap_or(0u32);

            let current_adjacency = &self.adjacency_list[state.current_node as usize];
            let next_to_visit = current_adjacency
                [(next_to_visit_start as usize)..current_adjacency.len()]
                .iter()
                .enumerate()
                .find(|(_idx, node_id)| **node_id <= end_node_idx && !visited.contains(**node_id));

            if let Some((idx, node_id)) = next_to_visit {
                // Exit this node
                stack.push(state.with_visited_idx(idx as u32 + next_to_visit_start));
                visited.insert(state.current_node);
                visited.insert(*node_id);
                stack.push(DfsState {
                    current_node: *node_id,
                    visited_list_idx: None,
                })
            }
        }
        total_paths
    }
}

pub fn main(args: &Args) {
    let lines = read_input_lines(args.day as u32, args.input_tag.as_deref());
    let graph = Graph::from_edges(lines);
    match args.part {
        1 => {
            let total_paths = graph.scan_paths("you", "out");
            println!("Total paths from 'you' to 'out': {}", total_paths);
        }
        2 => {
            let mut nodes_in_path = [
                *graph.name_to_idx.get("svr").unwrap(),
                *graph.name_to_idx.get("dac").unwrap(),
                *graph.name_to_idx.get("fft").unwrap(),
                *graph.name_to_idx.get("out").unwrap(),
            ];
            nodes_in_path.sort();
            let mut path_counts: Vec<usize> = Vec::new();
            for i in 1..nodes_in_path.len() {
                let from_node = graph.get_name_by_idx(nodes_in_path[i - 1]);
                let to_node = graph.get_name_by_idx(nodes_in_path[i]);
                let path_count = graph.scan_paths(from_node, to_node);
                path_counts.push(path_count);
            }
            let total_paths: usize = path_counts.iter().product();
            println!(
                "Total paths from 'svr' to 'out' visiting 'dac' and 'fft': {}",
                total_paths
            );
        }
        _ => {
            println!("Part {} is not yet implemented", args.part);
        }
    }
}
