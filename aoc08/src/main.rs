use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::fs::read_to_string;

type Node = [char; 3];

fn to_node(s: &str) -> (Node, [Node; 2]) {
    let re = Regex::new(r"(?P<node>...) = \((?P<out1>...), (?P<out2>...)\)").unwrap();
    let caps = re.captures(s).unwrap();

    let node: Node = caps.name("node").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();
    let out1: Node = caps.name("out1").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();
    let out2: Node = caps.name("out2").unwrap().as_str().chars().collect::<Vec<char>>().try_into().unwrap();

    (node, [out1, out2])
}
    
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].as_str();
    let data = read_to_string(&filename).unwrap();

    let mut lines = data.lines();

    let directions: Vec<usize> =
        lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    lines.next();

    let nodes: HashMap<Node, [Node; 2]> =
        lines
        .map(to_node)
        .collect();

    let mut positions: Vec<Node> =
        nodes
        .keys()
        .filter(|node| node[2] == 'A')
        .map(|node| *node)
        .collect();

    let mut count: usize = 0;

    let mut starts: Vec<usize> = Vec::new();
    let mut loop_lengths: Vec<usize> = Vec::new();
    let mut z_offsets_in_loop: Vec<usize> = Vec::new();
    
    for start_position in positions.iter() {
        let mut node = start_position;
        let mut count = 0;
        let mut order: Vec<Node> = Vec::new();
        let mut seen: HashMap<(Node, usize), usize> = HashMap::new(); 
        let mut z_position = 0;

        loop {
            if node[2] == 'Z' {
                z_position = count;
            }
            let entry = seen.entry((*node, count % directions.len()));
            match entry {
                Entry::Occupied(x) => { 
                    break;
                }
                Entry::Vacant(x) => {
                    x.insert(count);
                }
            }
            order.push(*node);
            node = &nodes[node][directions[count % directions.len()]];
            count += 1;
        }

        let loop_start = seen[&(*node, count % directions.len())];

        dbg!(count, z_position);
        starts.push(loop_start);
        loop_lengths.push(count - loop_start);
        z_offsets_in_loop.push(z_position - loop_start);
        //dbg!(loop_start, &order);

        assert!(z_position == count - loop_start);
    }
    dbg!(&starts, &loop_lengths, &z_offsets_in_loop);

   
    let mut counts: Vec<usize> = starts.iter().zip(z_offsets_in_loop.iter()).map(|(a, b)| a+b).collect();

    //let mut counts: Vec<usize> = vec![counts2[0]];
    
    while counts.iter().min() != counts.iter().max() {
        let min = counts.iter().enumerate().min_by_key(|(_, count)| *count).unwrap().0;
        let max = counts.iter().enumerate().max_by_key(|(_, count)| *count).unwrap().0;

        assert!(counts[min] < counts[max]);       
        //dbg!(min, counts[min], max, counts[max]);
        counts[min] += loop_lengths[min];
    }

    dbg!(&counts);
}
