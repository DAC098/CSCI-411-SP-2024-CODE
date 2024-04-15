use common;

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

fn graph_main() {
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

type Distance = i64;
type Cost = i64;

#[derive(Debug, Clone)]
struct HotelCost {
    cost: Cost,
    hotel: usize,
}

impl std::default::Default for HotelCost {
    fn default() -> Self {
        HotelCost {
            cost: 0,
            hotel: 0,
        }
    }
}

#[derive(Debug)]
struct CostResult {
    result: Vec<HotelCost>,
    largest: usize,
}

fn calc_hotel_distances(hotels: &[Distance], travel: Distance) -> CostResult {
    let mut memory: Vec<HotelCost> = vec![HotelCost::default(); hotels.len() + 1];
    let mut largest = 0;

    for memory_index in 1..memory.len() {
        memory[memory_index].cost = (travel - hotels[memory_index - 1]).pow(2);
        memory[memory_index].hotel = memory_index;

        for hotel_index in 0..(memory_index - 1) {
            let penalty = (travel - (hotels[memory_index - 1] - hotels[hotel_index])).pow(2);
            let prev = memory[hotel_index].cost + penalty;

            if prev < memory[memory_index].cost {
                memory[memory_index].cost = prev;
                memory[memory_index].hotel = hotel_index + 1;
            }
        }

        let check = common::get_int_len(memory[memory_index].cost);

        if check > largest {
            largest = check;
        }
    }

    CostResult {
        result: memory,
        largest: largest as usize,
    }
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut travel = 0;
    let mut expected = 0;
    let mut hotels: Vec<Distance> = Vec::new();

    {
        let check = lines.next()
            .expect("no header specified")
            .expect("failed to read input from stdin");

        let values = common::parse_line::<i64>(&check)
            .expect("failed to parse header line");

        if values.len() != 2 {
            panic!("expected to integers in header line");
        }

        if values[0] <= 0 {
            panic!("invalid expected hotels amount: {}", values[0]);
        }

        expected = values[0] as usize;
        travel = values[1];

        hotels.reserve(expected);
    }

    let mut hotel_largest: usize = 0;

    for line in lines {
        let valid = line.expect("failed to read input from stdin");

        let Ok(dist) = valid.parse() else {
            panic!("failed to parse hotel distance. line: \"{valid}\"");
        };

        let len = common::get_int_len(dist);

        if len > hotel_largest {
            hotel_largest = len;
        }

        hotels.push(dist);
    }

    println!("travel: {travel}");

    if hotels.len() != hotels.capacity() {
        panic!("number of hotels does not match expected amount. expected: {expected}");
    }

    let calculated= calc_hotel_distances(&hotels, travel);
    let result = calculated.result;
    let largest = if hotel_largest > calculated.largest {
        hotel_largest
    } else {
        calculated.largest
    };
    let spacer = " ".repeat(largest);

    print!("index:");

    for index in 0..result.len() {
        if index == 0 {
            print!(" {spacer}");
        } else {
            print!(" {:largest$}", index - 1);
        }
    }

    print!("\n dist:");

    for index in 0..result.len() {
        if index == 0 {
            print!(" {spacer}");
        } else {
            print!(" {:largest$}", hotels[index - 1]);
        }
    }

    print!("\n cost:");

    for index in 0..result.len() {
        print!(" {:largest$}", result[index].cost);
    }

    print!("\n prev:");

    for index in 0..result.len() {
        print!(" {:largest$}", result[index].hotel);
    }

    print!("\n");

    let mut next_index = result.len() - 1;
    let mut visited = Vec::new();

    while next_index != 0 {
        visited.push(next_index);

        if result[next_index].hotel == next_index {
            break;
        } else {
            next_index = result[next_index].hotel;
        }
    }

    visited.reverse();

    print!("Hotels to visit:");

    for hotel in visited {
        print!(" {hotel}");
    }

    println!("\nTotal penalty: {}", result[result.len() - 1].cost);
}
