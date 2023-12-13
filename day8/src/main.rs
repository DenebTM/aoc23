use std::{collections::HashMap, io};

use num::integer::lcm;
use regex::Regex;

type NodeName = String;
type Node = (
    /* self */ NodeName,
    (/* left */ NodeName, /* right */ NodeName),
);

fn get_steps(
    seq: &str,
    nodes: &HashMap<NodeName, (NodeName, NodeName)>,
    first: &str,
    is_last: impl Fn(&str) -> bool,
) -> usize {
    let mut current = first;
    let mut steps = 0;
    while !is_last(current) {
        let inst = seq.chars().nth(steps % seq.len()).unwrap();

        let next = nodes.get(current).unwrap();
        current = if inst == 'L' { &next.0 } else { &next.1 };

        steps += 1;
    }

    steps
}

fn main() {
    let mut seq: String = String::new();
    let mut nodes: Vec<Node> = Vec::new();

    let stdin = io::stdin();
    let mut buf = String::new();
    let mut line_num = 1;

    let node_regex = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        if line_num == 1 {
            seq = buf.trim().to_string();
        } else if line_num > 2 {
            if let Some(captures) = node_regex.captures(&buf) {
                let caps: Vec<NodeName> = captures
                    .iter()
                    .skip(1)
                    .flatten()
                    .map(|c| c.as_str().to_string())
                    .collect();

                let node: Node = (caps[0].clone(), (caps[1].clone(), caps[2].clone()));
                nodes.push(node);
            }
        }

        buf.clear();
        line_num += 1;
    }
    let nodes: HashMap<NodeName, (NodeName, NodeName)> = nodes.iter().cloned().collect();

    // part 1
    let first = "AAA";
    let last = "ZZZ";
    let steps = get_steps(&seq, &nodes, first, |current| current == last);
    println!("part1: {}", steps);

    // part 2: lowest common multiple
    let all_first: Vec<_> = nodes.iter().filter(|(name, _)| &name[2..] == "A").collect();

    let factors: Vec<usize> = all_first
        .iter()
        .map(|&(first, _)| get_steps(&seq, &nodes, first, |current| &current[2..] == "Z"))
        .collect();

    let steps: usize = factors.iter().fold(1, |acc, &factor| lcm(acc, factor));

    println!("part2: {}", steps);
}
