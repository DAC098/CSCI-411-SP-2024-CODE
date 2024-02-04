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

            int u = std::get<1>(line_result);
            int v = std::get<2>(line_result);
            Node* u_ptr;
            Node* v_ptr;

            try {
                u_ptr = known_nodes.at(u);
            } catch(std::out_of_range const& err) {
                Node u_node;
                u_node.value = u;
                u_node.visited = false;
                u_node.scc = {0, false};

                std::size_t index = graph.nodes.size();

                graph.nodes.push_back(u_node);
                u_ptr = &graph.nodes[index];

                known_nodes[u] = u_ptr;
            }

            try {
                v_ptr = known_nodes.at(v);
            } catch(std::out_of_range const& err) {
                Node v_node;
                v_node.value = v;
                v_node.visited = false;
                v_node.scc = {0, false};

                std::size_t index = graph.nodes.size();

                graph.nodes.push_back(v_node);
                v_ptr = &graph.nodes[index];

                known_nodes[v] = v_ptr;
            }

            Edge edge;
            edge.u = u_ptr;
            edge.v = v_ptr;

            graph.edges.push_back(edge);
            graph.neighbors[u].push_back(v_ptr);
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
