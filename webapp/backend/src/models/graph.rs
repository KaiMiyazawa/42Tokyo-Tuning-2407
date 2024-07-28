// models/graph.rs
use sqlx::FromRow;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// 辺のそれぞれの頂点を表す
// それぞれ判別id , x , yが振られている
#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

// /辺のそれぞれの頂点を表す
// それぞれの辺の判別idと重みが加えられている
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
    //空の配列を作成
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        // ここでは与えられたノード（頂点）の数分回してノードを初期化
        while self.nodes.len() <= node.id as usize {
            self.nodes.push(Node {
                id: self.nodes.len() as i32,
                x: 0,
                y: 0,
            });
            // ここでは隣接グラフの追加
            self.edges.push(vec![]);
        }
        self.nodes[node.id as usize] = node.clone();
    }

    pub fn add_edge(&mut self, edge: Edge) {
        //元のコードと同じように逆側の頂点とその初期値を追加
        self.edges[edge.node_a_id as usize].push(edge.clone());
        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        // 辺と頂点を重み（ここでは距離を追加）
        self.edges[reverse_edge.node_a_id as usize].push(reverse_edge);
    }

    // ダイクストラの処理
    // from_node_idは探索スタート地点[i]
    // to_node_idは指定された探索地点のゴールをさす
    pub fn dijkstra(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        // 無限で初期化
        let mut distances = vec![i32::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();

        // distanceとheapを初期化している。
        // distancehaは最短ルートの距離（重み）を格納するも
        // heepはここでは優先度queを指す
        distances[from_node_id as usize] = 0;
        // / { スタートからの距離（重み）　, スタートindex}
        heap.push((Reverse(0), from_node_id));

        // whileが,!enptyになるまで回す
        while let Some((Reverse(current_distance), current_node)) = heap.pop() {
            // 最速距離を更新できないのでスキップ
            if current_distance > distances[current_node as usize] {
                continue;
            }
            // 隣接の探索
            for edge in &self.edges[current_node as usize] {
                // グラフの現在時から行ける次の頂点
                let next_node = edge.node_b_id;
                // 現在の地点を含めた次の重み
                let next_distance = current_distance + edge.weight;
                // 最初は無限であるため、通り、そこから点々として頂点の大小比較が始まる
                if next_distance < distances[next_node as usize] {
                    // 最速距離を更新
                    distances[next_node as usize] = next_distance;
                    // 次の探索地点を追加
                    heap.push((Reverse(next_distance), next_node));
                }
            }
        }
        // ここで今回のゴールまでの最短経路を返す
        distances[to_node_id as usize]
    }
}
