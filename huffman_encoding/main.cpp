// CSCI 411 - Spring 2024
// Huffman encoding algorithm
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the Huffman encoding coding problem

#include <iostream>
#include <memory>
#include <unordered_map>
#include <queue>
#include <functional>
#include <bitset>

/*********************************************************************
 * A simple struct to keep track of information in a Huffman tree    *
 * count - int - the number of accurrences associated with this node *
 * w - string - the characters contributing to the occurrences       *
 * left - HuffmanNode* - the left child of this node                 *
 * right - HuffmanNode* - the right child of this node               *
 * *******************************************************************/
struct HuffNode {
    std::int8_t ch;
    std::uint64_t count;
    bool is_leaf;
    HuffNode* left;
    HuffNode* right;

    HuffNode(std::int8_t ch, std::uint64_t count) :
        ch(ch), count(count), is_leaf(true),
        left(nullptr), right(nullptr)
    {}

    HuffNode(HuffNode* left, HuffNode* right) :
        ch('\0'),
        count(left->count + right->count),
        is_leaf(false),
        left(left), right(right)
    {}
};

struct HuffNodeCmp {
    bool operator()(const HuffNode* a, const HuffNode* b) {
        return a->count > b->count;
    }
};

struct Trace {
    HuffNode* node;
    std::uint64_t depth;

    Trace(HuffNode* node, std::uint64_t depth) :
        node(node), depth(depth)
    {}
};

/**************************************************************************************
 * Generate a Huffman encoding for the characters of a string given the string itself *
 * s - string - the string to encode                                                  *
 * return - int - the number of bits (1s and 0s) required for a Huffman encoding of s *
 * ************************************************************************************/
std::uint64_t huffman_encoding(std::string& encode){
    std::vector<HuffNode> nodes;
    std::unordered_map<std::int8_t, std::uint64_t> frequency;
    std::priority_queue<HuffNode*, std::vector<HuffNode*>, HuffNodeCmp> queue;

    for (std::int8_t ch : encode) {
        frequency[ch] += 1;
    }

    if (frequency.size() == 1) {
        return (std::uint64_t)encode.size();
    }

    nodes.reserve(frequency.size() * 2 - 1);

    for (auto& pair : frequency) {
        nodes.push_back(HuffNode(pair.first, pair.second));
        queue.push(&nodes[nodes.size() - 1]);
    }

    while (queue.size() > 1) {
        auto left = queue.top();
        queue.pop();
        auto right = queue.top();
        queue.pop();

        nodes.push_back(HuffNode(left, right));
        queue.push(&nodes[nodes.size() - 1]);
    }

    std::queue<Trace> bfs_queue;
    bfs_queue.push(Trace(queue.top(), 0));
    queue.pop();

    std::uint64_t calc_size = 0;

    while (!bfs_queue.empty()) {
        auto& curr = bfs_queue.front();

        if (curr.node == nullptr) {
            bfs_queue.pop();
            continue
        }

        if (curr.node->is_leaf) {
            calc_size += curr.node->count * curr.depth;
        } else {
            bfs_queue.push(Trace(curr.node->left, curr.depth + 1));
            bfs_queue.push(Trace(curr.node->right, curr.depth + 1));
        }

        bfs_queue.pop();
    }

    return calc_size;
}

int main() {
    //get the string that we want to encode
    std::cout << "Enter a sequence of space separated strings to encode: ";

    std::string words = "";

    std::getline(std::cin, words);

    //encode the string using a Huffman encoding
    //print the total number of bits required for the encoding
    std::uint64_t result = huffman_encoding(words);

    std::cout << "The number of bits required to represent this string with a Huffman encoding: " << result << "\n";

    return 0;
}
