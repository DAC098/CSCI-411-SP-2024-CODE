/*
 * this is somewhat everywhere with how to access nodes in the graph.
 * sometimes it is by pointer other times it is by index.
 */
#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <stdexcept>
#include <vector>
#include <utility>
#include <map>
#include <set>
#include <queue>

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
    std::size_t value = 0;
    int dist = 0;
    bool inf = true;
    bool visited = false;

    void set_dist(int value) {
        this->dist = value;
        this->inf = false;
    }

    std::size_t id() {
        return this->value + 1;
    }
};

/**
 * an edge between two nodes
 */
struct Edge {
    Node* u = nullptr;
    Node* v = nullptr;
    int weight = 1;
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

bool set_contains(std::set<size_t> &to_check, const std::size_t& key) {
    std::set<std::size_t>::iterator iter = to_check.find(key);

    return iter != to_check.end();
}

void calc_graph(Options& options, Graph &graph) {
    std::set<std::size_t> in_negative_cycle;
    std::set<std::size_t> cycle_set;
    std::size_t iters = graph.nodes.size() - 1;

    // bfs node queue
    std::queue<std::size_t> queue;

    for (Node &src : graph.nodes) {
        cycle_set.clear();

        if (options.verbose) {
            std::cout << "source: " << src.id() << "\n";
        }

        // run bellman-ford
        for (Node &n : graph.nodes) {
            n.dist = 0;
            n.inf = false;
            n.visited = false;
        }

        src.set_dist(0);

        for (std::size_t count = 0; count < iters; ++count) {
            if (options.verbose) {
                std::cout << "iteration: " << count << "\n";
            }

            for (Edge &edge : graph.edges) {
                if (options.verbose) {
                    std::cout << "    " << edge.u->id() << " -> " << edge.v->id() << " w: " << edge.weight << " | u.dist: ";

                    if (edge.u->inf) {
                        std::cout << "inf";
                    } else {
                        std::cout << edge.u->dist;
                    }

                    std::cout << " | v.dist: ";

                    if (edge.v->inf) {
                        std::cout << "inf";
                    } else {
                        std::cout << edge.v->dist;
                    }
                }

                // because the infinity is indicated by a flag on the node
                // the check is different than the original algorithm

                // if u is at infinity the check will always be false, skip
                if (edge.u->inf) {
                    continue;
                }

                if (edge.v->inf) {
                    edge.v->set_dist(edge.u->dist + edge.weight);

                    if (options.verbose) {
                        std::cout << " | setting v dist: " << edge.v->dist;
                    }
                } else if (edge.v->dist > edge.u->dist + edge.weight) {
                    edge.v->set_dist(edge.u->dist + edge.weight);

                    if (options.verbose) {
                        std::cout << " | setting v dist: " << edge.v->dist;
                    }
                }

                if (options.verbose) {
                    std::cout << "\n";
                }
            }
        }

        if (options.verbose) {
            std::cout << "final iteration\n";
        }

        // this is a modification to check for the actual negative cycle
        for (Edge &edge : graph.edges) {
            if (options.verbose) {
                std::cout << "    " << edge.u->id() << " -> " << edge.v->id() << " w: " << edge.weight << " | u.dist: ";

                if (edge.u->inf) {
                    std::cout << "inf";
                } else {
                    std::cout << edge.u->dist;
                }

                std::cout << " | v.dist: ";

                if (edge.v->inf) {
                    std::cout << "inf";
                } else {
                    std::cout << edge.v->dist;
                }
            }

            if (edge.u->inf) {
                continue;
            }

            if (edge.v->inf) {
                if (set_contains(in_negative_cycle, edge.v->value)) {
                    continue;
                }

                cycle_set.insert(edge.v->value);

                if (options.verbose) {
                    std::cout << " in negative cycle";
                }
            } else if (edge.v->dist > edge.u->dist + edge.weight) {
                if (set_contains(in_negative_cycle, edge.v->value)) {
                    continue;
                }

                cycle_set.insert(edge.v->value);

                if (options.verbose) {
                    std::cout << " in negative cycle";
                }
            }

            if (options.verbose) {
                std::cout << "\n";
            }
        }

        if (cycle_set.size() == 0) {
            continue;
        }

        // there were nodes found to have a negative cycle so we will see
        // what other nodes we can reach from them
        if (options.verbose) {
            std::cout << "found negative cycles:\n";
        }

        for (std::size_t id : cycle_set) {
            in_negative_cycle.insert(id);

            if (options.verbose) {
                std::cout << "    " << graph.nodes[id].id() << " ->";
            }

            // BFS to determine what nodes are connected to the given 
            // source node.
            graph.nodes[id].visited = true;

            queue.push(id);

            while (!queue.empty()) {
                std::size_t current = queue.front();
                queue.pop();

                for (Node *n : graph.neighbors[current]) {
                    if (n->visited) {
                        continue;
                    }

                    n->visited = true;

                    queue.push(n->value);
                    in_negative_cycle.insert(n->value);

                    if (options.verbose) {
                        std::cout << " " << graph.nodes[n->value].id();
                    }
                }
            }
        }

        if (options.verbose) {
            std::cout << "\n";
        }
    }

    if (!in_negative_cycle.empty()) {
        // output any nodes found to be a part of a negative cycle
        if (options.verbose) {
            std::cout << "all nodes reachable in negative cycle:";
        }

        bool first = true;

        for (std::size_t index : in_negative_cycle) {
            if (first) {
                std::cout << graph.nodes[index].id();
                first = false;
            } else {
                std::cout << " " << graph.nodes[index].id();
            }
        }

        std::cout << "\n";
    }
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

    calc_graph(options, graph);

    return 0;
}
