// CSCI 411 - Spring 2024
// Assignment 9 Skeleton
// Author: Carter Tillquist
// Feel free to use all, part, or none of this code for the coding problem on assignment 9.

#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <sstream>

enum class Direction {
    LEFT,
    RIGHT
};

std::ostream& operator<<(std::ostream& out, const Direction& self) {
    switch (self) {
        case Direction::LEFT:
            out << 'L';
            break;
        case Direction::RIGHT:
            out << 'R';
            break;
        default:
            __builtin_unreachable();
    }

    return out;
}

template <typename T>
struct StateAction {
    T dest;
    char write;
    Direction dir;

    StateAction(T dest, char write, Direction dir) :
        dest(dest), write(write), dir(dir)
    {}
};

struct State {
    bool accepting;
    char symbol;
    std::map<char, StateAction<State*>> transitions;

    State(char symbol) :
        accepting(false), symbol(symbol), transitions({})
    {}
};

std::ostream& operator<<(std::ostream& out, const State& self) {
    out << self.symbol << " accept: " << self.accepting << " transitions:\n";

    for (const auto& action : self.transitions) {
        out << "    " << action.first << " => "
            << action.second.dest->symbol
            << " \"" << action.second.write << "\" "
            << action.second.dir << "\n";
    }

    return out;
}

struct TuringMachine {
    // q0
    char start_state;
    // B
    char blank_symbol;
    // sigma
    std::set<char> input_symbols;
    // gamma
    std::set<char> tape_symbols;

    TuringMachine() :
        blank_symbol('\0')
    {}

    std::map<char, State> states;
};

std::ostream& operator<<(std::ostream& out, const TuringMachine& self) {
    out << "states:\n";

    for (const auto& pair : self.states) {
        out << pair.second;
    }

    out << "start_state: " << self.start_state << "\n"
        << "blank_symbol: " << self.blank_symbol << "\n";

    out << "input_symbols:";

    for (const char& ch : self.input_symbols) {
        out << " " << ch;
    }

    out << "\ntape_symbols:";

    for (const char& ch : self.tape_symbols) {
        out << " " << ch;
    }

    return out;
}

struct Input {
    std::string line;
    std::istringstream iss;

    Input() {}

    void get_stdin() {
        std::getline(std::cin, this->line);
        this->iss.clear();
        this->iss.str(this->line);
    }

    void prompt_stdin(const char* prompt) {
        std::cout << prompt;
        std::cout.flush();

        this->get_stdin();
    }
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
TuringMachine getInput(std::string &w) {
    std::string holding;

    Input input;
    TuringMachine machine;

    input.prompt_stdin("Enter a string for the Turning machine ot use as input: ");
    input.iss >> w;

    input.prompt_stdin("Enter space separated state names: ");

    char state_value;

    while (input.iss >> state_value) {
        machine.states.insert({state_value, State(state_value)});
    }

    input.prompt_stdin("Enter the start state: ");
    input.iss >> machine.start_state;

    input.prompt_stdin("Enter a blank symbol for the tape: ");
    machine.blank_symbol = input.line[0];

    input.prompt_stdin("Enter space separated accepting states: ");

    while (input.iss >> state_value) {
        auto iter = machine.states.find(state_value);

        if (iter != machine.states.end()) {
            iter->second.accepting = true;
        } else {
            std::cout << "unknown state: " << state_value << "\n";
        }
    }

    input.prompt_stdin("Enter symbols of the input alphabet separated by spaces: ");

    while (input.iss >> holding) {
        machine.input_symbols.insert(holding[0]);
    }

    input.prompt_stdin("Enter symbols of the tape alphabet separated by spaces: ");

    while (input.iss >> holding) {
        machine.tape_symbols.insert(holding[0]);
    }

    input.prompt_stdin("Enter the number of transitions in the Turing machine: ");
    int max = std::stoi(input.line);

    std::cout << "Enter one transition per line with the initial state, symbol being read on the tape, destination state, symbol written to the tape, and tape head direction (L or R) separated by spaces: ";
    std::cout.flush();

    while (max--) {
        input.get_stdin();

        char read;
        char dest;
        char write;
        char dir;

        Direction valid;

        input.iss >> state_value;
        input.iss >> read;

        input.iss >> dest;
        input.iss >> write;
        input.iss >> dir;

        switch (dir) {
            case 'R':
                valid = Direction::RIGHT;
                break;
            case 'L':
                valid = Direction::LEFT;
                break;
            default:
                std::cout << "unknown direction: " << dir << "\n";
                continue;
        }

        auto iter = machine.states.find(state_value);

        if (iter == machine.states.end()) {
            std::cout << "unknown state symbol: " << state_value << "\n";
            continue;
        }

        auto dest_iter = machine.states.find(dest);

        if (iter == machine.states.end()) {
            std::cout << "unknown dest state symbol: " << dest << "\n";
            continue;
        }

        iter->second.transitions.insert({read, StateAction<State*>(&(dest_iter->second), write, valid)});
    }

    std::cout << std::endl;

    return machine;
}

/***************************************************************
 * Print the current configuration of a Turing machine         *
 * curState - string - the current state of the finite control *
 * tape - string - a representation of the current tape        *
 * loc - int - the location of the read/write head             *
****************************************************************/
std::string printConfiguration(std::string curState, std::string tape, int loc){
  tape.insert(loc, curState);

  std::cout << "Config: " << tape << std::endl;

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
bool simulateTM(const std::string &w, TuringMachine& machine) {
  // YOUR CODE HERE
  return true;
}

/************************************************************************
 * Read a Turing machine definition from cin along with an input string *
 * Determine whether or not the Turing machine recognizes the string    *
*************************************************************************/
int main(){
    std::string w;

    TuringMachine machine = getInput(w);

    std::cout << machine << "\n";

    bool result = simulateTM(w, machine);

    std::cout << w << " is " << (result ? "" : "not ") << "in the language recognized by the Turing machine" << std::endl;

  return 0;
}
