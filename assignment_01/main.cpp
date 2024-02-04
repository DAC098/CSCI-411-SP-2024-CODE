#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <stdexcept>
#include <vector>
#include <utility>
#include <map>

struct GroupData {
    std::size_t count = 0;
    bool incoming = false;
    bool outgoing = false;
};

struct Node {
    int value = 0;
    bool visited = false;
    std::pair<std::size_t, bool> scc = {0, false};
};

struct Edge {
    Node* u = nullptr;
    Node* v = nullptr;

    Edge reverse() {
        Edge rtn;
        rtn.u = this->v;
        rtn.v = this->u;

        return rtn;
    }
};

typedef std::vector<Node*> NodePtrList;
typedef std::map<int, NodePtrList> NeighborMap;
typedef std::vector<Edge> EdgeList;
typedef std::pair<EdgeList, NeighborMap> EdgeNeighborPair;

struct Graph {
    std::vector<Node> nodes = {};
    std::vector<Edge> edges = {};
    std::map<int, NodePtrList> neighbors = {};

    EdgeNeighborPair reverse() {
        NeighborMap rev_neighbors;
        EdgeList rev_edges;
        rev_edges.reserve(this->edges.size());

        for (Edge edge : this->edges) {
            Edge rev_edge = edge.reverse();
            rev_neighbors[rev_edge.u->value].push_back(rev_edge.v);
            rev_edges.push_back(rev_edge);
        }

        return {rev_edges, rev_neighbors};
    }
};

std::tuple<int, int ,int> get_int_pair(std::string &line) {
    int first = 0;
    int second = 0;
    std::size_t processed;
    const char *str = line.c_str();

    try {
        first = std::stoi(str, &processed);
    } catch(std::invalid_argument const& err) {
        return {0, first, second};
    } catch(std::out_of_range const& err) {
        return {0, first, second};
    }

    if (processed == line.length()) {
        return {0, first, second};
    }

    str += processed;

    try {
        second = std::stoi(str, &processed);
    } catch(std::invalid_argument const& err) {
        return {1, first, second};
    } catch(std::out_of_range const& err) {
        return {1, first, second};
    }

    return {2, first, second};
}

std::pair<std::size_t, bool> int_to_size_t(int v) {
    if (v < 0) {
        return {0, false};
    }

    return {(std::size_t)((unsigned)v), true};
}

std::string spacer(std::size_t len) {
    std::string rtn;
    rtn.reserve(len * 2);

    for (std::size_t count = 0; count < len; ++count) {
        rtn.push_back(' ');
        rtn.push_back(' ');
    }

    return rtn;
}

void dfs_scc(Graph &graph, Node *v, NodePtrList &list, std::size_t depth) {
    v->visited = true;

    std::cout << spacer(depth) << v->value << " visiting\n";

    for (Node *u : graph.neighbors[v->value]) {
        if (!u->visited) {
            dfs_scc(graph, u, list, depth + 1);
        } else {
            std::cout << spacer(depth) << u->value << " visited\n";
        }
    }

    std::cout << spacer(depth) << "push: " << v->value << "\n";

    list.push_back(v);
}

void dfs_assign(
    NeighborMap &neighbors,
    Node *v,
    std::size_t scc
) {
    v->scc = {scc, true};

    for (Node *u : neighbors[v->value]) {
        if (!u->scc.second) {
            dfs_assign(neighbors, u, scc);
        }
    }
}

void scc_graph(Graph &graph) {
    NodePtrList list;

    for (std::size_t index = 0; index < graph.nodes.size(); ++index) {
        if (!graph.nodes[index].visited) {
            dfs_scc(graph, &graph.nodes[index], list, 0);
        }
    }

    std::vector<GroupData> groups;
    EdgeNeighborPair reverse = graph.reverse();

    for (std::size_t index = list.size(); index--;) {
        if (!list[index]->scc.second) {
            std::size_t id = groups.size();

            dfs_assign(
                reverse.second,
                list[index],
                id
            );

            groups.push_back({});
        }
    }

    std::cout << "nodes:\n";

    for (Node u : graph.nodes) {
        std::cout << "    " << u.value << "[" << u.scc.first << "] -> ";

        for (Node *v : graph.neighbors[u.value]) {
            std::cout << v->value << ",";

            if (u.scc.first == v->scc.first) {
                continue;
            }

            groups[v->scc.first].incoming = true;
            groups[u.scc.first].outgoing = true;
        }

        groups[u.scc.first].count += 1;

        std::cout << "\n";
    }

    std::size_t group_a = 0;
    std::size_t group_b = 0;
    std::size_t group_c = 0;

    for (std::size_t id = 0; id < groups.size(); ++id) {
        std::cout << id << ": " << groups[id].count << " nodes";

        if (groups[id].incoming) {
            std::cout << " | incoming";
        }

        if (groups[id].outgoing) {
            std::cout << " | outgoing";
        }

        std::cout << "\n";

        if (groups[id].incoming && groups[id].outgoing) {
            group_c += groups[id].count;
        } else if (groups[id].outgoing) {
            group_a += groups[id].count;
        } else if (groups[id].incoming) {
            group_b += groups[id].count;
        } else {
            group_c += groups[id].count;
        }
    }

    std::cout << "group A: " << group_a
        << "\ngroup B: " << group_b
        << "\ngroup C: " << group_c << "\n";
}

int main() {
    std::string line;
    std::tuple<int, int, int> line_result;
    Graph graph;

    if (std::getline(std::cin, line)) {
        line_result = get_int_pair(line);

        int found = std::get<0>(line_result);

        if (found != 2) {
            std::cout << "invalid graph line provided: \"" << line << "\"\n";
            return 1;
        }

        int nodes = std::get<1>(line_result);
        int edges = std::get<2>(line_result);

        if (nodes <= 0) {
            std::cout << "amount of nodes specified is 0\n";
            return 1;
        }

        if (edges < 0) {
            std::cout << "amount of edges is less than 0\n";
            return 1;
        }

        graph.nodes.reserve((std::size_t)((unsigned)nodes));
        graph.edges.reserve((std::size_t)((unsigned)edges));

        for (int count = 0; count < nodes; ++count) {
            Node n;
            n.value = count + 1;
            n.visited = false;
            n.scc = {0, false};

            graph.nodes.push_back(n);
        }
    }

    {
        std::map<int, Node*> known_nodes;
        int found = 0;

        for (std::string line; std::getline(std::cin, line);) {
            line_result = get_int_pair(line);

            found = std::get<0>(line_result);

            if (found != 2) {
                std::cout << "invalid graph edge: \"" << line << "\"\n";
                return 1;
            }

            std::pair<std::size_t, bool> u_result = int_to_size_t(std::get<1>(line_result));
            std::pair<std::size_t, bool> v_result = int_to_size_t(std::get<2>(line_result));

            if (!u_result.second || !v_result.second) {
                std::cout << "invalid graph node: \"" << line << "\"\n";
                return 1;
            }

            std::size_t u = u_result.first - 1;
            std::size_t v = v_result.first - 1;

            if (u > graph.nodes.size() || v > graph.nodes.size()) {
                std::cout << "invalid graph node: \"" << line << "\"\n";
                return 1;
            }

            Edge edge;
            edge.u = &graph.nodes[u];
            edge.v = &graph.nodes[v];

            graph.edges.push_back(edge);
            graph.neighbors[graph.nodes[u].value].push_back(&graph.nodes[v]);
        }
    }

    std::cout << "nodes:\n";

    for (Node node : graph.nodes) {
        std::cout << "    " << node.value << " -> ";

        for (Node *v : graph.neighbors[node.value]) {
            std::cout << v->value << ",";
        }

        std::cout << "\n";
    }

    scc_graph(graph);

    return 0;
}
