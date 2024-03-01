use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    visited: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            visited: false,
        }
    }
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
}

impl From<(usize, usize)> for Edge {
    fn from((u,v): (usize, usize)) -> Self {
        Edge { u, v }
    }
}

type NodeList = Vec<Node>;
type NodeRefList = Vec<usize>;
type EdgeList = Vec<Edge>;
type NeighborMap = Vec<NodeRefList>;

#[derive(Debug)]
struct Graph {
    nodes: NodeList,
    edges: EdgeList,
    neighbors: NeighborMap,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: NodeList::new(),
            edges: EdgeList::new(),
            neighbors: NeighborMap::new(),
        }
    }
}

fn calc_graph(graph: Graph) {
    let mut stack = Vec::new();
    stack.push(0);

    while let Some(node) = stack.pop() {
        if graph.nodes[node].visited {
            continue;
        }

        let mut pushed_stack = false;

        for children in &graph.neighbors[node] {
            if graph.nodes[children].visited {
                continue;
            }

            stack.push(*children);
            pushed_stack = true;
        }

        if pushed_stack {
            contin
    }
}

fn calc_graph_recurse(
    node: usize,
    graph: &mut Graph,
    calced_nodes: &mut Vec<Option<usize>>,
) -> usize {
    if graph.neighbors[node].is_empty() {
        // base case: leaf
        calced_nodes[node] = Some(0);
        return 0;
    }

    for child in &graph.neighbors[node] {
        
    }
}

fn main() {
    let total_nodes = 15;
    let edges: EdgeList = vec![
        Edge::from((0,1)),
        Edge::from((0,2)),
        Edge::from((0,3)),
        Edge::from((1,4)),
        Edge::from((2,5)),
        Edge::from((3,6)),
        Edge::from((3,7)),
        Edge::from((3,6)),
        Edge::from((4,9)),
        Edge::from((4,10)),
        Edge::from((5,11)),
        Edge::from((5,12)),
        Edge::from((12,13)),
        Edge::from((12,14)),
    ];

    let mut graph = Graph::new();
    let mut calced_edges = vec![Node::<usize>; total_nodex];

    graph.nodes = vec![Node::default(); total_nodes];
    graph.neighbors = vec![Vec::new(); total_nodes];

    for edge in &edges {
        graph.neighbors[edge.u].push(edge.v);
    }

    graph.edges = edges;
}
