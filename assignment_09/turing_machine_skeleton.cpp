// CSCI 411 - Spring 2024
// Assignment 9 Skeleton
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the coding problem on assignment 9.

#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <sstream>
using namespace std;

/**********************************************************************************************
 * A simple struct holding the destination, the tape symbol to be written, and the            *
 * direction for the tape head to move from some state and for some input in a Turing machine *
 * dest - string - the destination state                                                      *
 * write - char - the tape symbol to write                                                    *
 * dir - char - the direction to move the tape head, L or R                                   *
***********************************************************************************************/
struct Action{
  string dest;
  char write;
  char dir;
};

/*********************************************************
 * A function to read input for a Turing machine         *
 * w - string& - the string to run on the Turing machine *
 * Q - set<string>& - the set of states                  *
 * q0 - string& - the start state                        *
 * B - char& - the blank symbol                          *
 * A - set<string>& the set of accepting states          *
 * sigma - set<char>& - the set of input symbols         *
 * gamma - set<char>& - the set of tape symbols          *
 * delta - map<string, map <char, Action>>&              *
**********************************************************/
void getInput(string &w, set<string> &Q, string &q0, char &B, set<string> &A, set<char> &sigma, set<char> &gamma, map<string, map <char, Action>> &delta){
  cout << "Enter a string for the Turing machine to use as input: ";
  getline(cin, w);
  istringstream iss(w);
  iss >> w;

  string states = "";
  cout << "Enter space separated state names: ";
  getline(cin, states);
  iss.clear();
  iss.str(states);
  string add;
  while(iss >> add){ Q.insert(add); }

  cout << "Enter the start state: ";
  getline(cin, q0);
  iss.clear();
  iss.str(q0);
  iss >> q0;

  string line = "";
  cout << "Enter a blank symbol for the tape: ";
  getline(cin, line);
  B = line[0];

  string accepting = "";
  cout << "Enter space separated accepting states: ";
  getline(cin, accepting);
  iss.clear();
  iss.str(accepting);
  while(iss >> add){ A.insert(add); }

  string alphabet = "";
  cout << "Enter symbols of the input alphabet separated by spaces: ";
  getline(cin, alphabet);
  iss.clear();
  iss.str(alphabet);
  while(iss >> add){ sigma.insert(add[0]); }

  string tape = "";
  cout << "Enter symbols of the tape alphabet separated by spaces: ";
  getline(cin, tape);
  iss.clear();
  iss.str(tape);
  while(iss >> add){ gamma.insert(add[0]); }

  string m = "";
  cout << "Enter the number of transitions in the Turing machine: ";
  getline(cin, m);

  string transition = "";
  cout << "Enter one transition per line with the initial state, symbol being read on the tape, destination state, symbol written to the tape, and tape head direction (L or R) separated by spaces: ";
  for (int i = 0; i < stoi(m); i++){
    getline(cin, transition);
    iss.clear();
    iss.str(transition);
    string init;
    char read;
    Action act;
    iss >> init;
    iss >> read;
    iss >> act.dest;
    iss >> act.write;
    iss >> act.dir;
    delta[init][read] = act;
  }
  cout << endl;
}

/***************************************************************
 * Print the current configuration of a Turing machine         *
 * curState - string - the current state of the finite control *
 * tape - string - a representation of the current tape        *
 * loc - int - the location of the read/write head             *
****************************************************************/
string printConfiguration(string curState, string tape, int loc){
  tape.insert(loc, curState);
  cout << "Config: " << tape << endl;
  return tape;
}

/***************************************************************
 * Run the given Turing machine on the provided string         *
 * Print configurations of the Turing machine at each step     *
 * w - const string& - the string to run on the Turing machine *
 * Q - set<string>& - the set of states                        *
 * q0 - string& - the start state                              *
 * B - char& - the blank symbol                                *
 * A - set<string>& the set of accepting states                *
 * sigma - set<char>& - the set of input symbols               *
 * gamma - set<char>& - the set of tape symbols                *
 * delta - map<string, map <char, Action>>&                    *
****************************************************************/
// states are names
bool simulateTM(const string &w, set<string> &Q, string &q0, char &B, set<string> &A, set<char> &sigma, set<char> &gamma, map<string, map <char, Action>> &delta){
  // YOUR CODE HERE
}

/************************************************************************
 * Read a Turing machine definition from cin along with an input string *
 * Determine whether or not the Turing machine recognizes the string    *
*************************************************************************/
int main(){
  string w;
  set<string> Q;
  string q0;
  char B;
  set<string> A;
  set<char> sigma;
  set<char> gamma;
  map<string, map <char, Action>> delta;
  getInput(w, Q, q0, B, A, sigma, gamma, delta);

  bool result = simulateTM(w, Q, q0, B, A, sigma, gamma, delta);
  cout << w << " is " << (result ? "" : "not ") << "in the language recognized by the Turing machine" << endl;

  return 0;
}
