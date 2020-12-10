use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

struct DirectedAcyclicGraph {
    nodes: BTreeSet<usize>,
    edges: HashMap<usize, Vec<usize>>,
}

impl DirectedAcyclicGraph {
    fn new() -> Self {
        DirectedAcyclicGraph {
            nodes: BTreeSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, input_joltage: usize, output_joltage: usize) {
        if let Some(adapters) = self.edges.get_mut(&input_joltage) {
            adapters.push(output_joltage);
        } else {
            let mut adapters = Vec::new();
            adapters.push(output_joltage);
            self.edges.insert(input_joltage, adapters);
        }
    }

    fn add_node(&mut self, node: usize) {
        self.nodes.insert(node);
    }
}

fn add_adapter(dag: &mut DirectedAcyclicGraph, output_joltage: usize) {
    dag.add_node(output_joltage);
    if output_joltage >= 1 {
        dag.add_edge(output_joltage - 1, output_joltage);
    }
    if output_joltage >= 2 {
        dag.add_edge(output_joltage - 2, output_joltage);
    }
    if output_joltage >= 3 {
        dag.add_edge(output_joltage - 3, output_joltage);
    }
}

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> DirectedAcyclicGraph {
    let mut dag = DirectedAcyclicGraph::new();

    // add the start node (power outlet)
    dag.add_node(0);

    // add the adapters
    let mut max_joltage = 0;
    for line in input.lines() {
        let joltage = line.parse().unwrap();
        max_joltage = max(max_joltage, joltage);
        add_adapter(&mut dag, joltage);
    }

    // add the end node (device)
    let device_rating = max_joltage + 3;
    add_adapter(&mut dag, device_rating);

    dag
}

#[aoc(day10, part1)]
fn part1(adapters: &DirectedAcyclicGraph) -> Option<usize> {
    let mut differences = [0; 3];
    let mut current = 0;
    for adapter in adapters.nodes.iter().skip(1) {
        let difference = adapter - current;
        match difference {
            1 | 2 | 3 => differences[difference - 1] += 1,
            _ => return None,
        }
        current = *adapter;
    }

    Some(differences[0] * differences[2])
}

#[aoc(day10, part2)]
fn part2(adapters: &DirectedAcyclicGraph) -> usize {
    let mut factors: HashMap<usize, usize> = HashMap::new();

    let mut permutations = 0;
    for adapter in adapters.nodes.iter().rev() {
        if let Some(neighbors) = adapters.edges.get(adapter) {
            permutations = 0;
            for neighbor in neighbors {
                permutations += factors.get(neighbor)?;
            }
            factors.insert(*adapter, permutations);
        } else {
            factors.insert(*adapter, 1);
        }
    }

    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "16
10
15
5
1
11
7
19
6
12
4";
    const INPUT_LARGE: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part1_small_example() {
        assert_eq!(part1(&parse_input_day10(INPUT_SMALL)), Some(7 * 5));
    }

    #[test]
    fn part1_large_example() {
        assert_eq!(part1(&parse_input_day10(INPUT_LARGE)), Some(22 * 10));
    }

    #[test]
    fn part2_small_example() {
        assert_eq!(part2(&parse_input_day10(INPUT_SMALL)), Some(8));
    }
    #[test]
    fn part2_large_example() {
        assert_eq!(part2(&parse_input_day10(INPUT_LARGE)), Some(19208));
    }
}
