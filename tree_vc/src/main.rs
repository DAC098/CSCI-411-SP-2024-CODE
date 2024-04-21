use std::io::Write as _;

use common;

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    include: bool,
    visited: bool,
}

impl Node {
    fn new(id: usize) -> Self {
        Node {
            id,
            include: false,
            visited: false,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: 0,
            include: false,
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

// WARNING: if the graph provided is big enough then this will cause a
// stack overflow and the program will crash
fn calc_graph(
    node: usize,
    node_list: &mut NodeList,
    neighbors: &NeighborMap,
    calced: &mut Vec<Calc>,
) -> usize {
    node_list[node].visited = true;

    let mut with_root = 1;
    let mut without_root = 0;
    let mut rtn_grand = 0;
    let mut included = 0;

    for child in &neighbors[node] {
        if node_list[*child].visited {
            continue;
        }

        included += calc_graph(*child, node_list, neighbors, calced);

        with_root += calced[*child].best;
        rtn_grand += calced[*child].best;
        without_root += 1 + calced[*child].child;
    }

    calced[node].child = rtn_grand;

    if with_root <= without_root {
        node_list[node].include = true;
        calced[node].best = with_root;
        included += 1;
    } else {
        calced[node].best = without_root;
    }

    included
}

struct IterState<'a> {
    node: usize,
    with_root: usize,
    without_root: usize,
    rtn_grand: usize,
    neighbors: std::iter::Peekable<std::slice::Iter<'a, usize>>,
}

fn calc_graph_iter(
    node: usize,
    node_list: &mut NodeList,
    neighbors: &NeighborMap,
    calced: &mut Vec<Calc>,
) -> usize {
    let mut included = 0;
    let mut queue = Vec::new();
    queue.push(IterState {
        node,
        with_root: 1,
        without_root: 0,
        rtn_grand: 0,
        neighbors: neighbors[node].iter()
            .peekable(),
    });

    while let Some(mut curr) = queue.pop() {
        let mut push_queue = None;

        node_list[curr.node].visited = true;

        // we do not want to advance the iterator until we know that it has
        // already been calculated so we will peek at the value before
        // advancing the iterator
        while let Some(neighbor) = curr.neighbors.peek() {
            if !node_list[**neighbor].visited {
                push_queue = Some(**neighbor);

                break;
            }

            let neighbor = curr.neighbors.next().unwrap();

            if let Some(next_queue) = queue.last() {
                // since we are working with an undirected graph we have to
                // make sure that we do not count the node that we came from
                // which should be the last node in the queue
                if *neighbor == next_queue.node {
                    continue;
                }
            }

            curr.with_root += calced[*neighbor].best;
            curr.rtn_grand += calced[*neighbor].best;
            curr.without_root += 1 + calced[*neighbor].child;
        }

        if let Some(neighbor) = push_queue {
            // we need to keep track of the current node still since we still
            // might have untraversed neighbors
            queue.push(curr);
            queue.push(IterState {
                node: neighbor,
                with_root: 1,
                without_root: 0,
                rtn_grand: 0,
                neighbors: neighbors[neighbor].iter()
                    .peekable(),
            });

            continue;
        }

        calced[curr.node].child = curr.rtn_grand;

        if curr.with_root <= curr.without_root {
            node_list[curr.node].include = true;
            calced[curr.node].best = curr.with_root;
            included += 1;
        } else {
            calced[curr.node].best = curr.without_root;
        }
    }

    included
}

fn main() {
    let mut iterative = true;
    let mut lines = std::io::stdin().lines();
    let mut line_result: Vec<usize> = Vec::with_capacity(2);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut neighbors = Vec::new();
    let mut calced = Vec::new();

    {
        print!("Enter the number of nodes and the number of edges: ");
        std::io::stdout()
            .flush()
            .expect("failed to flush to stdout");

        let line = lines.next()
            .expect("missing node and edge counts")
            .expect("failed to retrieve data from stdin");

        if !common::parse_line_fill(&line, &mut line_result) {
            panic!("failed to parse node and edge counts");
        }

        if line_result[0] <= 1 {
            panic!("number of nodes is less or equal to 1");
        }

        if line_result[1] <= 1 {
            panic!("number of edges is less or equal tot 1");
        }

        nodes.reserve(line_result[0]);
        edges.reserve(line_result[1]);
        neighbors.reserve(line_result[0]);

        calced.reserve(line_result[0]);

        for index in 0..line_result[0] {
            nodes.push(Node::new(index + 1));
            neighbors.push(Vec::new());

            calced.push(Calc::new());
        }
    }

    let mut start = None;

    println!("Enter the edges {{u,v}}: ");

    while let Some(line) = lines.next() {
        if edges.len() == edges.capacity() {
            break;
        }

        let given = line.expect("failed to retrieve data from stdin");

        line_result.clear();

        if !common::parse_line_fill(&given, &mut line_result) {
            panic!("failed to parse edge value");
        }

        if line_result[0] == 0 {
            panic!("edge u value is invalid: {} line: \"{given}\"", line_result[0]);
        }

        if line_result[1] == 0 {
            panic!("edge v value is invalid: {} line: \"{given}\"", line_result[1]);
        }

        let u = line_result[0] - 1;
        let v = line_result[1] - 1;

        if start.is_none() {
            start = Some(u);
        }

        edges.push(Edge::from((u,v)));

        neighbors[u].push(v);
        neighbors[v].push(u);
    }

    let start = start.unwrap();

    let result = if iterative {
        calc_graph_iter(start, &mut nodes, &neighbors, &mut calced)
    } else {
        calc_graph(start, &mut nodes, &neighbors, &mut calced)
    };

    println!("The minimum vertex cover size: {result}");
}
