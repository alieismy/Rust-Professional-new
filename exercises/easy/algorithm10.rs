/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

impl UndirectedGraph {
    // 添加一些辅助方法
    pub fn get_neighbors(&self, node: &str) -> Option<&Vec<(String, i32)>> {
        self.adjacency_table.get(node)
    }

    pub fn node_count(&self) -> usize {
        self.adjacency_table.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges().len() / 2  // 因为是无向图，每条边都被计算了两次
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        if self.adjacency_table_mutable().contains_key(node) {
            return true;
        }
        self.adjacency_table_mutable().insert(String::from(node), vec![]);
        true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        if !self.contains(from) {
            self.add_node(from);
        }
        if !self.contains(to) {
            self.add_node(to);
        }
        self.adjacency_table_mutable()
            .get_mut(from)
            .unwrap()
            .push((String::from(to), weight));
        self.adjacency_table_mutable()
            .get_mut(to)
            .unwrap()
            .push((String::from(from), weight));
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
