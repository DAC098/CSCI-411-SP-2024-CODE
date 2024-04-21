// CSCI 411 - Spring 2024
// Tree vertex cover algorithm
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the tree vertex cover coding problem

#include <iostream>
#include <vector>
using namespace std;

/*********************************************************************************************
 * Find the size of a minimum vertex cover in the tree represented by the adjacency matrix A *
 * A - vector<vector<int>> - an adjacency matrix A representing a tree                       *
 * return - int - the size of a minimum vertex cover for this tree                           *
 * *******************************************************************************************/
int treeVCSize(vector<vector<int>> A){
  // YOUR CODE HERE
  // I recommend using memoization for this problem
  // Consider writing a recursive function with an argument representing the tree (the adjacency matrix A),
  // the root of the subtree being considered, the node that was considered last, a vector<int> &C holding 
  // the size of minimum vertex covers of subtrees rooted at each node in the tree
}

int main(){
  //get the number of nodes and the number of edges in the tree
  cout << "Enter the number of nodes and the number of edges: ";
  int n = -1, m = -1;
  cin >> n >> m;

  //make an adjacency matrix for the tree
  //in this case, A holds integers, not Node structs, representing node labels
  //furthermore, the indexing starts at 0 (the first node is node 0)
  cout << "Enter the edges {u,v}: " << endl;
  vector<vector<int>> A(n);
  for (int x = 0; x < m; x++){
    int u = -1, v = -1;
    cin >> u >> v;
    A[u-1].push_back(v-1);
    A[v-1].push_back(u-1);
  }

  //find the minimum size of a vertex cover of the tree represented by A and print it
  cout << "The minimum vertex cover size: ";
  cout << treeVCSize(A) << endl;

  return 0;
}


