use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::read_to_string;

// #[derive(Default)]
// struct Nodes {
//     strings: BTreeMap<String, Box<str>>,
// }
//
// impl Nodes {
//     fn intern(&mut self, s: &str) -> &str {
//         match self.strings.entry(s.to_string()) {
//             Entry::Occupied(entry) => &entry.get(),
//             Entry::Vacant(mut entry) => {
//                 let interned = s.to_string().into_boxed_str();
//                 entry.insert(interned);
//                 &*interned
//             }
//         }
//     }
// }

struct Connection {
    from: String,
    to: Vec<String>,
}

struct Community {
    nodes: BTreeSet<usize>,
    sum_in: usize,
    sum_tot: usize,
}

impl Community {
    fn from(i: usize, node: &Node) -> Self {
        Self {
            nodes: BTreeSet::from([i]),
            sum_in: node.connections.get(&node.index).cloned().unwrap_or_default(),
            sum_tot: node.degree,
        }
    }
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
    community: usize,
    //map of connected node number to weight
    connections: BTreeMap<usize, usize>,
    degree: usize,
}

impl Node {
    fn new(index: usize, connections: &BTreeMap<usize, usize>) -> Self {
        let degree = connections.iter().map(|(_, weight)| weight).sum();
        let community = index;
        Self { index, community, connections: connections.clone(), degree }
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
    //names.sort_by_key(|name| name.parse::<u32>().unwrap());
    names.dedup();

    let mut connections: Vec<BTreeMap<usize, usize>>
        = vec![BTreeMap::default(); names.len()];

    for connection in &input {
        let from = names.iter().position(|s| s == &connection.from).unwrap();
        for to_str in &connection.to {
            let to = names.iter().position(|s| s == to_str).unwrap();
            connections[from].insert(to, 1);
            connections[to].insert(from, 1);
        }
    }

    let mut nodes: Vec<_> = (0..names.len())
        .map(|i| Node::new(i, &connections[i]))
        .collect();

    for i in 0.. {
        //println!("Pass {} nodes {}", i, nodes.len());
        let len = nodes.len();
        nodes = louvain_pass(&nodes);
        print_dot(&nodes);
        if nodes.len() == len {
            break;
        }
    }

    print_dot(&nodes);

    // for (i, community) in communities.iter().enumerate() {
    //     if community.nodes.len() > 0 {
    //         println!("Community {}: {}", i, community.nodes.len());
    //     }
    // }

    // Louvain method


    // bbz ->
    // mxd ->
    // brd ->
}

fn louvain_pass(n: &[Node]) -> Vec<Node> {
    let mut nodes = n.to_vec();

    let mut communities: Vec<_> = (0..nodes.len())
        .map(|i| Community::from(i, &nodes[i]))
        .collect();

    let m = nodes
        .iter()
        .map(|node| node.degree)
        .sum::<usize>();

    let mut moved = true;
    while moved {
        moved = false;
        for i in 0..nodes.len() {
            let mut max_delta_Q = f64::MIN;
            let mut max_delta_Q_community = 0;
            for (&j, &weight) in nodes[i].connections.iter() {
                if nodes[i].community == nodes[j].community {
                    continue;
                }

                let delta_Q =
                    delta_Q(&nodes[i],
                            &communities[nodes[i].community],
                            &communities[nodes[j].community],
                            m);

                if delta_Q > max_delta_Q {
                    max_delta_Q = delta_Q;
                    max_delta_Q_community = nodes[j].community;
                }
            }

            if max_delta_Q > 0.0 {
                let Q_old = Q(&nodes, m);
                let C_old = &mut communities[nodes[i].community];
                assert!(C_old.sum_in == sum_in(C_old, &nodes));
                assert!(C_old.sum_tot == sum_tot(C_old, &nodes));
                C_old.nodes.remove(&i);
                C_old.sum_in -= connections_to_community(&nodes[i], C_old) * 2;
                C_old.sum_tot -= nodes[i].degree;

                let C_new = &mut communities[max_delta_Q_community];
                C_new.nodes.insert(i);
                C_new.sum_in += connections_to_community(&nodes[i], C_new) * 2;
                C_new.sum_tot += nodes[i].degree;

                let ni = &mut nodes[i];
                ni.community = max_delta_Q_community;

                assert!(C_new.sum_in == sum_in(C_new, &nodes));
                assert!(C_new.sum_tot == sum_tot(C_new, &nodes));
                let Q_new = Q(&nodes, m);

                println!("{} {}", max_delta_Q, Q_new - Q_old);

                moved = true;
            }
            // for (i, community) in communities.iter().enumerate() {
            //     if community.nodes.len() > 0 {
            //         println!("Community {}: {}", i, community.nodes.len());
            //     }
            // }
        }
    }

    communities_to_nodes(&mut communities, &nodes)
}

fn delta_Q(i: &Node, C_out: &Community, C_in: &Community, m: usize) -> f64 {
    let M = 2. * (m as f64);
    let mut delta_Q = 0.;
    {
        let C = C_in;
        let sum_in = C.sum_in as f64;
        let sum_tot = C.sum_tot as f64;
        let k_i = i.degree as f64;
        let k_i_in = connections_to_community(i, C) as f64;
        delta_Q += (sum_in + 2.0 * k_i_in) / M - ((sum_tot + k_i) / M).powi(2)
            - (sum_in / M - (sum_tot / M).powi(2) - (k_i / M).powi(2));
    }
    {
        let C = C_out;
        let sum_in = C.sum_in as f64;
        let sum_tot = C.sum_tot as f64;
        let k_i = i.degree as f64;
        let k_i_in = connections_to_community(i, C) as f64;
        delta_Q += (sum_in - 2.0 * k_i_in) / M - ((sum_tot - k_i) / M).powi(2)
            - (sum_in / M - (sum_tot / M).powi(2) + (k_i / M).powi(2));
    }

    delta_Q
}

fn Q(nodes: &[Node], m: usize) -> f64 {
    let mut sum = 0.;
    let M = 2. * (m as f64);
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if nodes[i].community != nodes[j].community {
                continue;
            }
            sum += nodes[i].connections.get(&j).cloned().unwrap_or_default() as f64;
            sum -= ((nodes[i].degree * nodes[j].degree) as f64) / M;
        }
    }
    sum / M
}

fn sum_tot(C: &Community, nodes: &[Node]) -> usize {
    C.nodes
        .iter()
        .map(|&i| nodes[i].degree)
        .sum()
}

fn sum_in(C: &Community, nodes: &[Node]) -> usize {
    C.nodes
        .iter()
        .map(|&i| nodes[i]
            .connections
            .iter()
            .filter(|(&j, &_weight)| nodes[i].community == nodes[j].community)
            .map(|(&_j, &weight)| weight)
            .sum::<usize>())
        .sum()
}

fn connections_to_community(n: &Node, C: &Community) -> usize {
    n.connections
        .iter()
        .filter(|(connection, &_weight)| C.nodes.contains(connection))
        .map(|(&_connection, &weight)| weight)
        .sum()
}

fn communities_to_nodes(communities: &[Community], nodes: &[Node]) -> Vec<Node> {
    for (n, i) in communities
        .iter()
        .enumerate()
        .filter(|&(_i, community)| !community.nodes.is_empty())
        .map(|(i, _community)| i)
        .enumerate() {
        let community = &communities[i];
        if community.nodes.len() == 0 {
            continue;
        }
        println!("{}: {}", n, community.nodes.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(" "));
    }

    let community_to_new_node_map = BTreeMap::<usize, usize>::from_iter(
        communities
            .iter()
            .enumerate()
            .filter(|&(_i, community)| !community.nodes.is_empty())
            .map(|(i, _community)| i)
            .enumerate()
            .map(|(n, i)| (i, n)));

    let new_node_to_community_map = BTreeMap::<usize, usize>::from_iter(
        community_to_new_node_map
            .iter()
            .map(|(&i, &n)| (n, i)));

    let mut new_nodes = vec![Node::default(); new_node_to_community_map.len()];

    for i in 0..new_nodes.len() {
        let c = new_node_to_community_map[&i];
        let community = &communities[c];
        for &old_node in &community.nodes {
            for (&old_node_connection, &weight) in &nodes[old_node].connections {
                let old_community = nodes[old_node_connection].community;
                let new_connection = community_to_new_node_map[&old_community];
                *new_nodes[i].connections.entry(new_connection).or_default() += weight;
                new_nodes[i].degree += weight;
                new_nodes[i].community = i;
                // if new_connection == c {
                //     *new_nodes[i].connections.entry(new_connection).or_default() += weight;
                //     new_nodes[i].degree += weight;
                // }
            }
        }
    }

    for (i, new_node) in new_nodes.iter_mut().enumerate() {
        if let Some(self_weight) = new_node.connections.get_mut(&i) {
            // *self_weight /= 2;
            // new_node.degree -= *self_weight;
        }
    }

    new_nodes
}

fn print_dot(nodes: &[Node]) {
    let mut seen = BTreeSet::new();
    println!("graph name {{");
    for (from, node) in nodes.iter().enumerate() {
        println!("{} [label=\"{}\"]", from, node.degree);
        for (&to, weight) in &node.connections {
            if !seen.contains(&to) {
                println!("{} -- {} [label=\"{}\"]", from, to, weight);
            }
        }
        seen.insert(from);
    }
    println!("}}");
}