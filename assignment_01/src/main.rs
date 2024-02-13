use std::default::Default;

#[derive(Debug)]
struct GroupData {
    count: usize,
    incoming: bool,
    outgoing: bool,
}

impl Default for GroupData {
    fn default() -> Self {
        GroupData {
            count: 0,
            incoming: false,
            outgoing: false,
        }
    }
}

#[derive(Debug)]
struct Node {
    visited: bool,
    scc: Option<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            visited: false,
            scc: None
        }
    }
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
}

impl Edge {
    fn reverse(&self) -> Edge {
        Edge {
            u: self.v,
            v: self.u,
        }
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

    fn reverse(&self) -> (EdgeList, NeighborMap) {
        let mut neighbors = vec![NodeRefList::new(); self.neighbors.len()];
        let mut edges = EdgeList::new();

        edges.reserve(self.edges.len());

        for edge in &self.edges {
            let rev = edge.reverse();

            neighbors[rev.u].push(rev.v);
            edges.push(rev);
        }

        (edges, neighbors)
    }
}

fn parse_int_line(line: &str) -> Option<Vec<i32>> {
    let split = line.split(' ');
    let mut rtn = Vec::new();

    for value in split {
        let Ok(parsed) = value.parse() else {
            return None;
        };

        rtn.push(parsed);
    }

    Some(rtn)
}

fn dfs_scc(nodes: &mut NodeList, neighbors: &NeighborMap, v: &usize, list: &mut NodeRefList) {
    nodes[*v].visited = true;

    for u in &neighbors[*v] {
        if !nodes[*u].visited {
            dfs_scc(nodes, neighbors, u, list);
        }
    }

    list.push(*v);
}

fn dfs_assign(nodes: &mut NodeList, rev_neighbors: &NeighborMap, v: &usize, scc: usize) {
    nodes[*v].scc = Some(scc);

    for u in &rev_neighbors[*v] {
        if nodes[*u].scc.is_none() {
            dfs_assign(nodes, rev_neighbors, u, scc);
        }
    }
}

fn calc_graph(mut graph: Graph) {
    let mut list = NodeRefList::new();

    for index in 0..graph.nodes.len() {
        if !graph.nodes[index].visited {
            dfs_scc(
                &mut graph.nodes,
                &graph.neighbors,
                &index,
                &mut list
            );
        }
    }

    let mut groups = Vec::new();
    let (_rev_edges, rev_neighbors) = graph.reverse();

    for index in list.iter().rev() {
        if graph.nodes[*index].scc.is_none() {
            let scc = groups.len();

            dfs_assign(
                &mut graph.nodes,
                &rev_neighbors,
                index,
                scc
            );

            groups.push(GroupData::default());
        }
    }

    for u in 0..graph.nodes.len() {
        let u_scc = (&graph.nodes[u].scc).unwrap();

        for v in &graph.neighbors[u] {
            let v_scc = (&graph.nodes[*v].scc).unwrap();

            if u_scc == v_scc {
                continue;
            }

            groups[v_scc].incoming = true;
            groups[u_scc].outgoing = true;
        }

        groups[u_scc].count += 1;
    }

    let mut group_a = 0usize;
    let mut group_b = 0usize;
    let mut group_c = 0usize;

    for group in groups {
        if group.incoming && group.outgoing {
            group_c += group.count;
        } else if group.outgoing {
            group_a += group.count;
        } else if group.incoming {
            group_b += group.count;
        } else {
            group_c += group.count;
        }
    }

    print!(
        "Number of nodes and number of edges: \nAdd {} edges: \n|A| = {}, |B| = {}, |C| = {}",
        graph.edges.len(),
        group_a,
        group_b,
        group_c,
    );
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut graph = Graph::new();
    let nodes;
    let edges;

    {
        let Some(check) = lines.next() else {
            panic!("no graph data specified");
        };

        let graph_line = check.expect("failed to read input from stdin");

        let Some(graph_data) = parse_int_line(&graph_line) else {
            panic!("invalid graph line provided: \"{}\"", graph_line);
        };

        if graph_data.len() < 2 {
            panic!("too few graph values: \"{}\"", graph_line);
        }

        nodes = {
            let Ok(check): Result<usize, _> = graph_data[0].try_into() else {
                panic!("amount of nodes specified is invalid: {}", graph_data[0]);
            };

            if check <= 0 {
                panic!("amount of nodes specified is invalid: {}", graph_data[0]);
            }

            check
        };

        edges = {
            let Ok(check): Result<usize, _> = graph_data[1].try_into() else {
                panic!("amount of edges specified is invalid: {}", graph_data[1]);
            };

            check
        };

        graph.nodes.reserve(nodes);
        graph.edges.reserve(edges);
        graph.neighbors.reserve(nodes);

        for _index in 0..nodes {
            graph.nodes.push(Node::default());
            graph.neighbors.push(Vec::new());
        }
    }

    while let Some(check) = lines.next() {
        let line = check.expect("failed to read input from stdin");

        let Some(found) = parse_int_line(&line) else {
            panic!("invalid graph edge: \"{}\"", line);
        };

        if found.len() != 2 {
            panic!("too few edge values: \"{}\"", line);
        }

        let Ok(mut u): Result<usize, _> = found[0].try_into() else {
            panic!("edge u value is invalid: {} \"{}\"", found[0], line);
        };
        let Ok(mut v): Result<usize, _> = found[1].try_into() else {
            panic!("edge v value is invalid: {} \"{}\"", found[1], line);
        };

        if u == 0 || v == 0 {
            panic!("edge u/v values are invalid: u: {} v: {} \"{}\"", u, v, line);
        }

        u -= 1;
        v -= 1;

        if u >= graph.nodes.len() {
            panic!("edge u index not found: {} \"{}\"", u, line);
        }

        if v >= graph.nodes.len() {
            panic!("edge v index not found: {} \"{}\"", v, line);
        }

        graph.edges.push(Edge { u, v });
        graph.neighbors[u].push(v);
    }

    calc_graph(graph);
}
