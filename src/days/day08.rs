use aoc::Context;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Graph {
    nodes: HashMap<usize, HashSet<usize>>, // node index -> list of neighbor indices
}

pub fn solve(ctx: &mut Context) {
    let mut coords = Vec::new();
    for line in ctx.input() {
        let mut split = line.split(',');
        coords.push(Coordinate {
            x: split.next().unwrap().parse::<i64>().unwrap(),
            y: split.next().unwrap().parse::<i64>().unwrap(),
            z: split.next().unwrap().parse::<i64>().unwrap(),
        });
    }

    let mut distances: HashMap<(usize, usize), f64> = HashMap::new(); // (index1, index2) -> distance
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = coords[i].euclidean_distance(&coords[j]);
            distances.insert((i, j), dist);
        }
    }

    // Order by distances ascending
    let mut sorted_distances: Vec<((usize, usize), f64)> = distances.into_iter().collect();
    sorted_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut graph = Graph {
        nodes: HashMap::new(),
    };
    coords.iter().enumerate().for_each(|(i, _)| {
        graph.nodes.insert(i, HashSet::new());
    });

    let mut problem_1_graph = graph.clone();
    let n = if ctx.is_example() { 10 } else { 1000 };
    connect_n_first_pairs(n, &sorted_distances, &mut problem_1_graph);

    let mut circuits = calculate_circuits(&problem_1_graph);
    // Order circuits by size descending
    circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));
    let result = circuits.iter().take(3).map(|c| c.len()).product::<usize>();
    ctx.set_sol1(result);

    // Find the minimum n that results in a single circuit
    let mut low = 1;
    let mut high = sorted_distances.len();
    while low < high {
        let mid = (low + high) / 2;
        let mut test_graph = graph.clone();
        connect_n_first_pairs(mid, &sorted_distances, &mut test_graph);
        let test_circuits = calculate_circuits(&test_graph);
        if test_circuits.len() == 1 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    // The pair that connects the last two circuits is at index low - 1
    let pair_index = low - 1;
    let (box1, box2) = sorted_distances[pair_index].0;
    let box1 = &coords[box1];
    let box2 = &coords[box2];
    ctx.set_sol2(box1.x * box2.x);
}

fn connect_n_first_pairs(n: usize, sorted_distances: &[((usize, usize), f64)], graph: &mut Graph) {
    sorted_distances
        .iter()
        .take(n)
        .for_each(|&((box1, box2), _)| {
            connect_boxes(box1, box2, graph);
        });
}

fn connect_boxes(box1: usize, box2: usize, graph: &mut Graph) {
    // Add connection both ways
    graph.nodes.get_mut(&box1).unwrap().insert(box2);
    graph.nodes.get_mut(&box2).unwrap().insert(box1);
}

fn calculate_circuits(graph: &Graph) -> Vec<HashSet<usize>> {
    // Build circuits by finding connected components
    let mut visited = HashSet::new();
    let mut circuits = Vec::new();
    for &node in graph.nodes.keys() {
        if visited.contains(&node) {
            continue;
        }
        // New circuit found
        let mut stack = vec![node];
        let mut circuit = HashSet::new();
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            circuit.insert(current);
            for &neighbor in &graph.nodes[&current] {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        circuits.push(circuit);
    }

    circuits
}
