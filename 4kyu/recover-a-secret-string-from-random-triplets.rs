// https://www.codewars.com/kata/recover-a-secret-string-from-random-triplets/train/rust
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

struct NodeInfo {
    in_degree: usize,
    first: Option<usize>,
}

impl NodeInfo {
    fn new() -> Self {
        Self {
            in_degree: 0,
            first: None,
        }
    }
}

struct ForwardStar<T> where T: Clone + Copy + Eq + Hash + PartialEq {
    nodes: HashMap<T, NodeInfo>,
    edges: Vec<(T, Option<usize>)>,
}

impl<T> ForwardStar<T> where T: Clone + Copy + Eq + Hash + PartialEq {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn node(&mut self, t: &T) -> &mut NodeInfo {
        self.nodes.entry(*t).or_insert(NodeInfo::new())
    }

    fn append(&mut self, source: &T, target: &T) {
        let source_node = self.node(source);
        self.edges.push((*target, source_node.first));
        source_node.first = Some(self.edges.len() - 1);
        self.node(target).in_degree += 1;
    }

    fn iterate<F>(&mut self, t: &T, f: F) where F: Fn(&T) {
        let mut current = self.node(t).first;
        while let Some(index) = current {
            f(&self.edges[index].0);
            current = self.edges[index].1;
        }
    }
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut graph: ForwardStar<char> = ForwardStar::new();
    for [u, v, w] in triplets {
        graph.append(&u, &v);
        graph.append(&v, &w);
    }
    if let Some((&first, ..)) = graph.nodes.iter().find(|(_, info)| info.in_degree == 0) {
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push(first);
        while let Some(current) = heap.pop() {
            visited.insert(current);
            graph.node(&current);
        }
    } else {
        panic!("cannot determine the beginning of the string");
    }
    String::new()
}
