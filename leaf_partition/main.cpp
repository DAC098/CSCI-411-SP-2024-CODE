// CSCI 411 - Spring 2024
// Leaf partition cover algorithm
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the leaf partition coding problem

#include <set>
#include <iostream>
#include <memory>
#include <vector>
#include <map>
#include <algorithm>

struct Node {
    std::size_t id;

    std::vector<Node*> neighbors;

    Node(std::size_t id) :
        id(id)
    {}

    bool is_major() {
        return this->neighbors.size() > 2;
    }

    bool is_leaf() {
        return this->neighbors.size() <= 1;
    }
};

typedef std::map<std::size_t, std::vector<std::size_t>> PartitionMap;

PartitionMap partition_leaves(const std::set<Node*>& leaves) {
    PartitionMap known_partitions;

    for (Node* leaf : leaves) {
        Node* prev = leaf;
        Node* curr = leaf;

        while (!curr->is_major()) {
            for (Node* neighbor : curr->neighbors) {
                if (neighbor == prev) {
                    continue;
                }

                prev = curr;
                curr = neighbor;
                break;
            }

            if (curr->is_leaf()) {
                break;
            }
        }

        if (!curr->is_major()) {
            continue;
        }

        known_partitions[curr->id].push_back(leaf->id);
    }

    return known_partitions;
}

int main() {
    // get the number of nodes and number of edges in the tree
    std::size_t n = 0, m = 0;

    std::cout << "Enter the number of nodes and the number of edges: ";

    std::cin >> n >> m;

    std::vector<Node> nodes;
    nodes.reserve(n);
    std::set<Node*> leaves;

    for (std::size_t index = 0; index < n; ++index) {
        nodes.push_back(Node(index + 1));
        leaves.insert(&nodes[index]);
    }

    std::cout << "Enter the edges {u,v}: " << std::endl;

    for (std::size_t index = 0; index < m; index++){
        std::size_t u = 0, v = 0;

        std::cin >> u >> v;

        u -= 1;
        v -= 1;

        nodes[u].neighbors.push_back(&nodes[v]);
        nodes[v].neighbors.push_back(&nodes[u]);

        if (!nodes[u].is_leaf()) {
            auto found = leaves.find(&nodes[u]);

            if (found != leaves.end()) {
                leaves.erase(found);
            }
        }

        if (!nodes[v].is_leaf()) {
            auto found = leaves.find(&nodes[v]);

            if (found != leaves.end()) {
                leaves.erase(found);
            }
        }
    }

    PartitionMap partitions = partition_leaves(leaves);

    std::cout << "The leaf partition: \n";

    for (auto pair : partitions) {
        std::cout << pair.first << " ";

        for (auto id : pair.second) {
            std::cout << id << " ";
        }

        std::cout << "\n";
    }

    return 0;
}
