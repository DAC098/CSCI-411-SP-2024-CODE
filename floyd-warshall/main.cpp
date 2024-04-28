// CSCI 411 - Spring 2024
// Floyd-Warshall algorithm
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the Floyd-Warshall coding problem

#include <iostream>
#include <vector>
#include <limits.h>

struct Mem {
    int value = 0;
    bool inf = true;

    Mem() :
        value(0), inf(true)
    {}
};

void floyd_warshall(std::vector<std::vector<Mem>>& matrix) {
    for (std::size_t k = 0; k < matrix.size(); ++k) {
        for (std::size_t i = 0; i < matrix.size(); ++i) {
            for (std::size_t j = 0; j < matrix.size(); ++j) {
                if (matrix[i][k].inf || matrix[k][j].inf) {
                    continue;
                }

                int value = matrix[i][k].value + matrix[k][j].value;

                if (matrix[i][j].inf || matrix[i][j].value > value) {
                    matrix[i][j].value = value;
                    matrix[i][j].inf = false;
                }
            }
        }
    }
}

int main() {
    //get the number of nodes and the number of edges in the graph
    std::size_t n = 0, m = 0;

    std::cout << "Enter the number of nodes and the number of edges separated by a space: ";

    std::cin >> n >> m;

    std::cout << "Enter m edges (u,v) and their weights: " << std::endl;

    std::vector<std::vector<Mem>> matrix(n, std::vector<Mem>(n, Mem()));

    for (std::size_t i = 0; i < matrix.size(); i++) {
        matrix[i][i].value = 0;
        matrix[i][i].inf = false;
    }

    for (std::size_t count = 0; count < m; count++){
        std::size_t i = 0, j = 0;
        int w = -1;

        //std::cout << count;

        std::cin >> i >> j >> w;

        //std::cout << "\n";

        matrix[i - 1][j - 1].value = w;
        matrix[i - 1][j - 1].inf = false;
    }

    //fill the distance matrix D using the Floyd-Warshall algorithm and print the result
    std::cout << "The distance matrix for G: " << std::endl;

    floyd_warshall(matrix);

    for (std::size_t i = 0; i < matrix.size(); i++){
        for (std::size_t j = 0; j < matrix[i].size(); j++){
            if (!matrix[i][j].inf) {
                std::cout << matrix[i][j].value << " ";
            } else {
                std::cout << "INF ";
            }
        }

        std::cout << std::endl;
    }

    return 0;
}

