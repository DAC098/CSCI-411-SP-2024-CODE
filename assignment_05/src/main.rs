#[derive(Debug, Clone)]
struct Node {
    include: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
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
        node_list[node].include = true;
        calced_nodes[node] = with_root;
    } else {
        calced_nodes[node] = without_root;
    }

    println!("{spacer}{} {} with: {with_root} without: {without_root}", node + 1, node_list[node].include);
}

#[derive(Clone)]
struct Calc {
    best: usize,
    child: usize,
}

impl Calc {
    fn new() -> Self {
        Calc {
            best: 0,
            child: 0
        }
    }
}

impl std::fmt::Display for Calc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.best, self.child)
    }
}

fn calc_graph_2(
    node: usize,
    node_list: &mut NodeList,
    neighbors: &NeighborMap,
    calced: &mut Vec<Calc>,
    depth: usize
) {
    let spacer = "| ".repeat(depth);

    println!("{spacer}calculating: {}", node + 1);

    let mut with_root = 1;
    let mut without_root = 0;
    let mut rtn_grand = 0;

    for child in &neighbors[node] {
        calc_graph_2(*child, node_list, neighbors, calced, depth + 1);

        with_root += calced[*child].best;
        rtn_grand += calced[*child].best;
        without_root += 1 + calced[*child].child;
    }

    calced[node].child = rtn_grand;

    if with_root <= without_root {
        node_list[node].include = true;
        calced[node].best = with_root;
    } else {
        calced[node].best = without_root;
    }

    println!("{spacer}{} {} with: {with_root} without: {without_root}", node + 1, node_list[node].include);
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

        graph.nodes = vec![Node::default(); total_nodes];
        graph.neighbors = vec![Vec::new(); total_nodes];

        for edge in &edges {
            graph.neighbors[edge.u].push(edge.v);
        }

        graph.edges = edges;

        {
            println!("calc_graph results:");

            let mut calced_edges = vec![0usize; total_nodes];

            calc_graph(0, &mut graph.nodes, &graph.neighbors, &mut calced_edges, 0);

            for node in 0..graph.nodes.len() {
                println!("{}: {} included: {}", node + 1, calced_edges[node], graph.nodes[node].include);
                graph.nodes[node].include = false;
            }
        }

        {
            println!("calc_graph_2 results:");

            let mut calced = vec![Calc::new(); total_nodes];

            calc_graph_2(0, &mut graph.nodes, &graph.neighbors, &mut calced, 0);

            for node in 0..graph.nodes.len() {
                println!("{}: {} included: {}", node + 1, calced[node], graph.nodes[node].include);
                graph.nodes[node].include = false;
            }
        }

        graph_count += 1;
    }
}
