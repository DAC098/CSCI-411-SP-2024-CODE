#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <stdexcept>
#include <vector>
#include <utility>
#include <map>

/**
 * this options available for the program
 */
struct Options {
    bool verbose = false;
};

/**
 * a single node in the graph
 */
struct Node {
    int value = 0;
    bool visited = false;
};

/**
 * an edge between two nodes
 */
struct Edge {
    Node* u = nullptr;
    Node* v = nullptr;
    int weight = 1;

    Edge reverse() {
        Edge rtn;
        rtn.u = this->v;
        rtn.v = this->u;

        return rtn;
    }
};

typedef std::vector<Node> NodeList;
typedef std::vector<Node*> NodePtrList;
typedef std::vector<NodePtrList> NeighborMap;
typedef std::vector<Edge> EdgeList;
typedef std::pair<EdgeList, NeighborMap> EdgeNeighborPair;

/**
 * contains the necessary information to represent a graph
 *
 * includes a map of neighbors for a given node based on the value of that node
 */
struct Graph {
    NodeList nodes = {};
    EdgeList edges = {};
    NeighborMap neighbors = {};

    /**
     * creates a reverse edge list and neighbors map
     */
    EdgeNeighborPair reverse() {
        NeighborMap rev_neighbors;
        EdgeList rev_edges;

        rev_neighbors.reserve(this->nodes.size());
        rev_edges.reserve(this->edges.size());

        for (Edge edge : this->edges) {
            Edge rev_edge = edge.reverse();

            rev_neighbors[rev_edge.u->value].push_back(rev_edge.v);
            rev_edges.push_back(rev_edge);
        }

        return {rev_edges, rev_neighbors};
    }
};

/**
 * parses a given string to extract a list of integers
 */
void get_int_list(std::string &line, std::vector<int> &list) {
    std::size_t processed;
    std::size_t total_processed;
    int parsed;
    const char *str = line.c_str();

    while (true) {
        try {
            parsed = std::stoi(str, &processed);
        } catch (std::invalid_argument const& err) {
            return;
        } catch (std::out_of_range const& err) {
            return;
        }

        total_processed += processed;

        if (total_processed == line.length()) {
            return;
        }

        str += processed;

        list.push_back(parsed);
    }
}

/**
 * converts an int to a size_t
 */
std::pair<std::size_t, bool> int_to_size_t(int v) {
    if (v < 0) {
        return {0, false};
    }

    return {(std::size_t)((unsigned)v), true};
}

/**
 * creates a string with len*2 spaces
 */
std::string spacer(std::size_t len) {
    std::string rtn;
    rtn.reserve(len * 2);

    for (std::size_t count = 0; count < len; ++count) {
        rtn.push_back(' ');
        rtn.push_back(' ');
    }

    return rtn;
}

int main(int argc, char* argv[]) {
    Options options;

    for (int index = 1; index < argc; ++index) {
        std::string str(argv[index]);

        if (str == "--verbose") {
            options.verbose = true;
        }
    }

    std::string line;
    std::vector<int> line_result;
    Graph graph;

    if (std::getline(std::cin, line)) {
        get_int_list(line, line_result);

        if (line_result.size() != 2) {
            std::cout << "invalid graph line provided: \"" << line << "\"\n";
            return 1;
        }

        if (line_result[0] <= 0) {
            std::cout << "amount of nodes specified is 0\n";
            return 1;
        }

        if (line_result[1] < 0) {
            std::cout << "amount of edges is less than 0\n";
            return 1;
        }

        std::size_t nodes_len = (std::size_t)((unsigned)line_result[0]);

        graph.nodes.reserve(nodes_len);
        graph.edges.reserve((std::size_t)((unsigned)line_result[1]));

        for (std::size_t count = 0; count < nodes_len; ++count) {
            Node n;
            n.value = count;
            n.visited = false;

            graph.nodes.push_back(n);
            graph.neighbors.push_back({});
        }
    }

    {
        for (; std::getline(std::cin, line);) {
            line_result.clear();

            get_int_list(line, line_result);

            if (line_result.size() < 2) {
                std::cout << "invalid graph edge: \"" << line << "\"\n";
                return 1;
            }

            std::pair<std::size_t, bool> u_result = int_to_size_t(line_result[0]);
            std::pair<std::size_t, bool> v_result = int_to_size_t(line_result[1]);

            if (!u_result.second || !v_result.second) {
                std::cout << "invalid graph node: \"" << line << "\"\n";
                return 1;
            }

            std::size_t u = u_result.first - 1;
            std::size_t v = v_result.first - 1;

            if (u > graph.nodes.size() || v > graph.nodes.size()) {
                std::cout << "invalid graph line: \"" << line << "\"\n";
                return 1;
            }

            if (graph.edges.size() == graph.edges.capacity()) {
                std::cout << "too many edges specified\n";
                return 1;
            }

            Edge edge;
            edge.u = &graph.nodes[u];
            edge.v = &graph.nodes[v];

            if (line_result.size() == 3) {
                edge.weight = line_result[2];
            }

            graph.edges.push_back(edge);
            graph.neighbors[u].push_back(&graph.nodes[v]);
        }
    }

    if (options.verbose) {
        std::cout << "nodes:\n";

        for (Node node : graph.nodes) {
            std::cout << "    " << node.value + 1 << " -> ";

            for (Node *v : graph.neighbors[node.value]) {
                std::cout << v->value + 1 << ",";
            }

            std::cout << "\n";
        }
    }

    return 0;
}
