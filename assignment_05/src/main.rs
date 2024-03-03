use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    visited: bool,
    include: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            visited: false,
            include: false,
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

fn calc_graph(
    node: usize,
    node_list: &mut NodeList,
    neighbors: &NeighborMap,
    calced_nodes: &mut Vec<usize>,
    depth: usize,
) {
    let spacer = "| ".repeat(depth);

    println!("{spacer}calculating: {}", node + 1);

    let mut with_root = 1;
    let mut without_root = 0;

    for child in &neighbors[node] {
        calc_graph(*child, node_list, neighbors, calced_nodes, depth + 1);

        with_root += calced_nodes[*child];
        without_root += 1;

        for grand in &neighbors[*child] {
            without_root += calced_nodes[*grand];
        }
    }

    if with_root <= without_root {
        println!("{spacer}including {} with: {with_root} without: {without_root}", node + 1);
        node_list[node].include = true;
        calced_nodes[node] = with_root;
    } else {
        println!("{spacer}excluding {} with: {with_root} without: {without_root}", node + 1);
        calced_nodes[node] = without_root;
    }
}

fn main() {
    let graph_list: Vec<(usize, EdgeList)> = vec![
        (15, vec![
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
        ]),
        (18, vec![
            Edge::from((0,1)),
            Edge::from((0,2)),
            Edge::from((0,3)),
            Edge::from((1,4)),
            Edge::from((1,5)),
            Edge::from((2,6)),
            Edge::from((3,7)),
            Edge::from((3,8)),
            Edge::from((3,9)),
            Edge::from((4,10)),
            Edge::from((4,11)),
            Edge::from((6,12)),
            Edge::from((6,13)),
            Edge::from((9,14)),
            Edge::from((9,15)),
            Edge::from((9,16)),
            Edge::from((9,17)),
        ]),
        (16, vec![
            Edge::from((0,1)),
            Edge::from((0,2)),
            Edge::from((0,3)),
            Edge::from((1,4)),
            Edge::from((1,5)),
            Edge::from((2,6)),
            Edge::from((3,7)),
            Edge::from((3,8)),
            Edge::from((3,9)),
            Edge::from((4,10)),
            Edge::from((4,11)),
            Edge::from((9,12)),
            Edge::from((9,13)),
            Edge::from((9,14)),
            Edge::from((9,15)),
        ])
    ];

    let mut graph_count = 0;

    for (total_nodes, edges) in graph_list {
        println!("graph: {}", graph_count + 1);

        let mut graph = Graph::new();
        let mut calced_edges = vec![0usize; total_nodes];

        graph.nodes = vec![Node::default(); total_nodes];
        graph.neighbors = vec![Vec::new(); total_nodes];

        for edge in &edges {
            graph.neighbors[edge.u].push(edge.v);
        }

        graph.edges = edges;

        calc_graph(0, &mut graph.nodes, &graph.neighbors, &mut calced_edges, 0);

        for node in 0..graph.nodes.len() {
            println!("{}: {} included: {}", node + 1, calced_edges[node], graph.nodes[node].include);
        }

        graph_count += 1;
    }
}
