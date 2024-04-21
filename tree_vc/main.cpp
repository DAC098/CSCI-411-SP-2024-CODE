#include <iostream>
#include <vector>

/**
 * a single node in the graph
 */
struct Node {
    std::size_t value;

    std::size_t best;
    std::size_t child;

    bool visited;
    bool include;

    std::vector<Node*> neighbors;

    Node(std::size_t value) :
        value(value), best(0), child(0), visited(false), include(false)
    {}
};

typedef std::vector<Node> NodeList;
typedef std::vector<Node*> NodePtrList;

struct IterState {
    Node* node;
    std::size_t with_root;
    std::size_t without_root;
    std::size_t rtn_grand;
    NodePtrList::iterator iter;
    NodePtrList::iterator end;

    IterState(Node* node) :
        node(node),
        with_root(1),
        without_root(0),
        rtn_grand(0),
        iter(node->neighbors.begin()),
        end(node->neighbors.end())
    {}
};

bool traverse_neighbors(IterState& curr, std::vector<IterState>& queue) {
    while (curr.iter != curr.end) {
        if (!(*curr.iter)->visited) {
            return false;
        }

        if (queue.size() > 1) {
            IterState& next = queue[queue.size() - 2];

            if (next.node == *curr.iter) {
                curr.iter += 1;

                continue;
            }
        }

        curr.with_root += (*curr.iter)->best;
        curr.rtn_grand += (*curr.iter)->best;
        curr.without_root += 1 + (*curr.iter)->child;

        curr.iter += 1;
    }

    return true;
}

int vertex_cover(Node* source, NodeList& nodes) {
    std::size_t included = 0;
    std::vector<IterState> queue;
    queue.push_back(IterState(source));

    while (!queue.empty()) {
        IterState& curr = queue.back();

        curr.node->visited = true;

        if (!traverse_neighbors(curr, queue)) {
            queue.push_back(IterState(*curr.iter));

            continue;
        }

        curr.node->child = curr.rtn_grand;

        if (curr.with_root <= curr.without_root) {
            curr.node->include = true;
            curr.node->best = curr.with_root;
            included += 1;
        } else {
            curr.node->best = curr.without_root;
        }

        queue.pop_back();
    }

    return included;
}

int main(int argc, char* argv[]) {
    std::size_t nodes_len = 0;
    std::size_t edges_len = 0;
    NodeList nodes;

    std::cout << "Enter the number of nodes and the number of edges: ";
    std::cin >> nodes_len >> edges_len;

    if (nodes_len <= 1) {
        std::cout << "number of nodes is less or equal to 1\n";
        return 1;
    }

    if (edges_len <= 1) {
        std::cout << "number of edges is less or equal to 1\n";
        return 1;
    }

    for (std::size_t index = 0; index < nodes_len; ++index) {
        nodes.push_back(Node(index));
    }

    std::cout << "Enter the edges {u,v}: \n";

    Node* source = nullptr;

    while (edges_len--) {
        std::size_t u;
        std::size_t v;

        std::cin >> u >> v;

        if (u == 0 || u > nodes_len) {
            std::cout << "edge u value is invalid: " << u << "\n";
            return 1;
        }

        if (v == 0 || v > nodes_len) {
            std::cout << "edge v value is invalid: " << v << "\n";
            return 1;
        }

        u -= 1;
        v -= 1;

        if (source == nullptr) {
            source = &nodes[u];
        }

        nodes[u].neighbors.push_back(&nodes[v]);
        nodes[v].neighbors.push_back(&nodes[u]);
    }

    std::size_t result = vertex_cover(source, nodes);

    std::cout << "The minimum vertex cover size: " << result << "\n";

    return 0;
}
