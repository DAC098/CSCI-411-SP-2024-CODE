use std::collections::HashMap;

use clap::Parser;

#[derive(Debug, Parser)]
struct Options {
    #[arg(short, long)]
    verbose: bool
}

#[derive(Debug)]
struct Node {
    index: usize,
    value: i32,
    visited: bool,
    scc: Option<usize>,
}

impl Node {
    fn new(index: usize) -> Node {
        let value = (index as i32) + 1;

        Node {
            index,
            value,
            visited: false,
            scc: None
        }
    }
}

#[derive(Debug)]
struct Edge<'a> {
    u: &'a Node,
    v: &'a Node,
}

impl<'a> Edge<'a> {
    fn reverse(&self) -> Edge<'a> {
        Edge {
            u: self.v,
            v: self.u,
        }
    }
}

type NodeList = Vec<Node>;
type NodeRefList<'a> = Vec<&'a Node>;
type EdgeList<'a> = Vec<Edge<'a>>;
type NeighborMap<'a> = Vec<NodeRefList<'a>>;

#[derive(Debug)]
struct Graph<'a> {
    nodes: NodeList,
    edges: EdgeList<'a>,
    neighbors: NeighborMap<'a>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            neighbors: Vec::new(),
        }
    }

    fn reverse(&self) -> (EdgeList<'a>, NeighborMap<'a>) {
        let mut neighbors = vec![Vec::new(); self.neighbors.len()];
        let mut edges = Vec::new();

        edges.reserve(self.edges.len());

        for edge in &self.edges {
            let rev = edge.reverse();

            neighbors[rev.u.index].push(rev.v);
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

fn calc_graph<'a>(opts: &Options, mut graph: Graph<'a>) {
}

fn main() {
    let options = Options::parse();

    println!("{:#?}", options);

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

        println!("nodes: {} | edges: {}", nodes, edges);

        graph.nodes.reserve(nodes);
        graph.edges.reserve(edges);
        graph.neighbors.reserve(nodes);

        for index in 0..nodes {
            graph.nodes.push(Node::new(index));
            graph.neighbors.push(Vec::new());
        }
    }

    while let Some(check) = lines.next() {
        let line = check.expect("failed to read input from stdin");

        println!("line: \"{}\"", line);

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

        let Some(u_ref) = graph.nodes.get(u) else {
            panic!("edge u index not found: {} \"{}\"", u, line);
        };
        let Some(v_ref) = graph.nodes.get(v) else {
            panic!("edge v index not found: {} \"{}\"", v, line);
        };

        graph.edges.push(Edge {
            u: u_ref,
            v: v_ref
        });
        graph.neighbors[u].push(&graph.nodes[v]);
    }

    println!("{:#?}", graph);

    let (rev_edges, rev_neighbors) = graph.reverse();

    println!("reverse edges: {:#?}\nreverse neighbors: {:#?}", rev_edges, rev_neighbors);
}
