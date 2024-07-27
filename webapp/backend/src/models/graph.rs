// models/graph.rs
use sqlx::FromRow;
use std::collections::BinaryHeap;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        while self.nodes.len() <= node.id as usize {
            self.nodes.push(Node { id: self.nodes.len() as i32, x: 0, y: 0 });
            self.edges.push(vec![]);
        }
        self.nodes[node.id as usize] = node.clone();
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges[edge.node_a_id as usize].push(edge.clone());
        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges[reverse_edge.node_a_id as usize].push(reverse_edge);
    }

    pub fn dijkstra(&self, start_id: i32, goal_id: i32) -> i32 {
        let mut distances = vec![i32::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();

        distances[start_id as usize] = 0;
        heap.push((std::cmp::Reverse(0), start_id));

        while let Some((std::cmp::Reverse(current_distance), current_node)) = heap.pop() {
            if current_node == goal_id {
                return current_distance;
            }

            if current_distance > distances[current_node as usize] {
                continue;
            }

            for edge in &self.edges[current_node as usize] {
                let next = edge.node_b_id;
                let next_distance = current_distance + edge.weight;

                if next_distance < distances[next as usize] {
                    distances[next as usize] = next_distance;
                    heap.push((std::cmp::Reverse(next_distance), next));
                }
            }
        }

        distances[goal_id as usize]
    }
}
