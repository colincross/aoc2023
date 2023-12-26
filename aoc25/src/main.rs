use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs::read_to_string;

use ordered_float::OrderedFloat;

struct Connection {
    from: String,
    to: Vec<String>,
}

impl Connection {
    fn from(s: &str) -> Self {
        let (from_str, to_list) = s.split_once(": ").unwrap();
        let from = from_str.to_string();
        let to = to_list
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Self { from, to }
    }

    fn all(&self) -> Vec<String> {
        let mut all = self.to.clone();
        all.push(self.from.clone());
        all
    }
}


#[derive(Clone, Default)]
struct Node {
    index: usize,
    //map of connected node number to weight
    connections: BTreeSet<usize>,
}

impl Node {
    fn new(index: usize, connections: &BTreeSet<usize>) -> Self {
        Self { index, connections: connections.clone() }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let input: Vec<_> = data
        .lines()
        .map(Connection::from)
        .collect();

    let mut names = input
        .iter()
        .map(|c| c.all())
        .flatten()
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();

    let mut connections: Vec<BTreeSet<usize>>
        = vec![BTreeSet::default(); names.len()];

    for connection in &input {
        let from = names.iter().position(|s| s == &connection.from).unwrap();
        for to_str in &connection.to {
            let to = names.iter().position(|s| s == to_str).unwrap();
            connections[from].insert(to);
            connections[to].insert(from);
        }
    }

    let nodes: Vec<_> = (0..names.len())
        .map(|i| Node::new(i, &connections[i]))
        .collect();

    let mut edge_map = BTreeMap::<(usize, usize), f64>::new();
    for node in &nodes {
        edge_scores(node, &nodes, &mut edge_map);
    }

    let mut edges = edge_map
        .iter()
        .collect::<Vec<_>>();
    edges.sort_by_key(|(_, &score)| OrderedFloat(score));

    let edges_to_remove = edges
        .iter()
        .rev()
        .take(3)
        .map(|(&edge, _score)| edge)
        .collect::<Vec<_>>();

    let sets = find_connected_sets(&remove_edges(&nodes, &edges_to_remove));

    assert!(sets.len() == 2);
    println!("{} * {} = {}",
             sets[0].len(),
             sets[1].len(),
             sets[0].len() * sets[1].len())
}

fn remove_edges(n: &[Node], edges: &[(usize, usize)]) -> Vec<Node> {
    let mut nodes = n.to_vec();

    for &edge in edges {
        nodes[edge.0].connections.remove(&edge.1);
        nodes[edge.1].connections.remove(&edge.0);
    }

    nodes
}

fn find_connected_sets(nodes: &[Node]) -> Vec<BTreeSet<usize>> {
    let mut sets = Vec::<BTreeSet<usize>>::new();

    for i in 0..nodes.len() {
        if sets.iter().any(|set| set.contains(&i)) {
            continue;
        }

        let mut set = BTreeSet::<usize>::new();
        let mut stack = Vec::<usize>::new();
        stack.push(i);
        while let Some(n) = stack.pop() {
            set.insert(n);
            for &dep in &nodes[n].connections {
                if !set.contains(&dep) {
                    stack.push(dep);
                }
            }
        }
        sets.push(set);
    }

    sets
}

#[allow(dead_code)]
fn print_dot(nodes: &[Node]) {
    let mut seen = BTreeSet::new();
    println!("graph name {{");
    for (from, node) in nodes.iter().enumerate() {
        for to in &node.connections {
            if !seen.contains(to) {
                println!("{} -- {}", from, to);
            }
        }
        seen.insert(from);
    }
    println!("}}");
}

fn edge_scores(node: &Node, nodes: &[Node], edge_map: &mut BTreeMap<(usize, usize), f64>) {
    let mut node_scores = vec![(usize::MAX, 0); nodes.len()];
    let mut predecessors = vec![BTreeSet::<usize>::new(); nodes.len()];
    let mut walk_order = Vec::<usize>::new();

    node_score_bfs(node, nodes, &mut node_scores, &mut predecessors, &mut walk_order);

    edge_score_bfs(nodes, &node_scores, &predecessors, &walk_order, edge_map);
}

fn node_score_bfs(start: &Node,
                  nodes: &[Node],
                  node_scores: &mut [(usize, usize)],
                  predecessors: &mut Vec<BTreeSet<usize>>,
                  walk_order: &mut Vec<usize>) {
    let mut queue = VecDeque::<&Node>::new();
    queue.push_back(start);
    node_scores[start.index] = (0, 0);
    while let Some(node) = queue.pop_front() {
        walk_order.push(node.index);
        let depth = node_scores[node.index].0 + 1;
        for &dep in &node.connections {
            let (node_depth, count) = &mut node_scores[dep];
            if depth < *node_depth {
                *node_depth = depth;
                queue.push_back(&nodes[dep]);
            }
            if depth == *node_depth {
                *count += 1;
                predecessors[dep].insert(node.index);
            }
        }
    }
}

fn edge_score_bfs(nodes: &[Node],
                  node_scores: &[(usize, usize)],
                  predecessors: &[BTreeSet<usize>],
                  walk_order: &[usize],
                  edge_map: &mut BTreeMap<(usize, usize), f64>) {
    let mut delta = vec![0.; nodes.len()];
    for &i in walk_order.iter().rev() {
        let coeff = (1. + delta[i]) / (node_scores[i].1 as f64);
        for &predecessor in &predecessors[i] {
            let c = node_scores[predecessor].1 as f64 * coeff;
            let edge = edge(i, predecessor);
            *edge_map.entry(edge).or_default() += c;
            delta[predecessor] += c;
        }
    }
}

fn edge(i: usize, j: usize) -> (usize, usize) {
    if i < j {
        (i, j)
    } else {
        (j, i)
    }
}