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

// we are doing this to get around the circular dependency, this is not the
// best but it works for this purpose
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
    std::string name;
    std::map<char, StateAction<State*>> transitions;

    State(std::string name) :
        accepting(false), name(name), transitions({})
    {}
};

std::ostream& operator<<(std::ostream& out, const State& self) {
    out << "name: \"" << self.name << "\" accepting: ";

    if (self.accepting) {
        out << "true";
    } else {
        out << "false";
    }

    out << " transitions:\n";

    for (const auto& action : self.transitions) {
        out << "    input: \"" << action.first
            << "\" => name: \"" << action.second.dest->name
            << "\" write: \"" << action.second.write
            << "\" dir: " << action.second.dir << "\n";
    }

    return out;
}

struct TuringMachine {
    char blank;
    State* start;
    std::set<char> input_symbols;
    std::set<char> tape_symbols;

    TuringMachine() :
        blank('\0'), start(nullptr)
    {}

    std::map<std::string, State> states;
};

std::ostream& operator<<(std::ostream& out, const TuringMachine& self) {
    out << "states:\n";

    for (const auto& pair : self.states) {
        out << pair.second;
    }

    out << "start: " << self.start->name << "\n"
        << "blank: " << self.blank << "\n";

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

TuringMachine getInput(std::string &w) {
    std::string holding;

    Input input;
    TuringMachine machine;

    // input word
    input.prompt_stdin("Enter a string for the Turing machine to use as input: ");
    input.iss >> w;

    // base state inputs
    input.prompt_stdin("Enter space separated state names: ");

    std::string state_value;

    while (input.iss >> state_value) {
        machine.states.insert({state_value, State(state_value)});
    }

    // start state
    input.prompt_stdin("Enter the start state: ");
    input.iss >> state_value;

    auto start_iter = machine.states.find(state_value);

    if (start_iter == machine.states.end()) {
        std::cout << "unknown start state: " << state_value << "\n";
        return machine;
    }

    machine.start = &start_iter->second;

    // blank symbol input
    input.prompt_stdin("Enter a blank symbol for the tape: ");
    input.iss >> machine.blank;

    // accepting states
    input.prompt_stdin("Enter space separated accepting states: ");

    while (input.iss >> state_value) {
        auto iter = machine.states.find(state_value);

        if (iter != machine.states.end()) {
            iter->second.accepting = true;
        } else {
            std::cout << "unknown state: " << state_value << "\n";
        }
    }

    // input symbols
    input.prompt_stdin("Enter symbols of the input alphabet separated by spaces: ");

    while (input.iss >> holding) {
        machine.input_symbols.insert(holding[0]);
    }

    // tape symbols
    input.prompt_stdin("Enter symbols of the tape alphabet separated by spaces: ");

    while (input.iss >> holding) {
        machine.tape_symbols.insert(holding[0]);
    }

    // transitions
    input.prompt_stdin("Enter the number of transitions in the Turing machine: ");
    int max = std::stoi(input.line);

    std::cout << "Enter one transition per line with the initial state, symbol being read on the tape, destination state, symbol written to the tape, and tape head direction (L or R) separated by spaces: ";
    std::cout.flush();

    while (max--) {
        input.get_stdin();

        char read;
        char write;
        char dir;
        std::string dest;

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

void print_tape(const std::string& tape, const std::string& state_name, const char& blank, const std::size_t& head_pos) {
    std::size_t index = 0;
    std::size_t end = tape.size() - 1;

    // skip over any leading blanks up to the first non blank position or we
    // encounter the current head position
    while (index < tape.size() && index != head_pos) {
        if (tape[index] != blank) {
            break;
        }

        index += 1;
    }

    // skip over any trailing blanks up to the first non blank position or we
    // encounter the current head position
    while (end != index && end != head_pos) {
        if (tape[end] != blank) {
            break;
        }

        end -= 1;
    }

    // since we are doing less than comparison we have to make sure to be one
    // larger than the last position other wise we will not print the last
    // character we need to show
    end += 1;

    std::cout << "Config: ";

    while (index < end) {
        if (index == head_pos) {
            std::cout << state_name;
        }

        std::cout << tape[index];
        index += 1;
    }

    std::cout << "\n";
}

bool simulateTM(const std::string &w, const TuringMachine& machine) {
    std::string tape = w;
    State* curr = machine.start;
    std::size_t head_pos = 0;

    while (true) {
        print_tape(tape, curr->name, machine.blank, head_pos);

        auto transition = curr->transitions.find(tape[head_pos]);

        if (transition == curr->transitions.end()) {
            break;
        }

        tape[head_pos] = transition->second.write;

        switch (transition->second.dir) {
            case Direction::LEFT:
                if (head_pos == 0) {
                    tape.insert(0, 1, machine.blank);
                    // we dont need to change the head position as we have
                    // shifted the tape right
                } else {
                    head_pos -= 1;
                }

                break;
            case Direction::RIGHT:
                if (head_pos == tape.size() - 1) {
                    tape.push_back(machine.blank);
                }

                head_pos += 1;
                break;
            default:
                __builtin_unreachable();
                break;
        }

        curr = transition->second.dest;
    }

    return curr->accepting;
}

int main(){
    std::string w;

    TuringMachine machine = getInput(w);

    bool result = simulateTM(w, machine);

    std::cout << w << " is ";

    if (!result) {
        std::cout << "not ";
    }

    std::cout << "in the language recognized by the Turing machine\n";

    return 0;
}
