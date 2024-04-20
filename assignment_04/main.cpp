#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <tuple>
#include <stdexcept>
#include <vector>
#include <utility>
#include <map>
#include <set>
#include <queue>

/**
 * parses a given string to extract a list of integers
 */
void get_int_list(std::string &line, std::vector<int> &list) {
    std::size_t processed;
    std::size_t total_processed;
    int parsed;
    const char *str = line.c_str();

    while (true) {
        try {
            parsed = std::stoi(str, &processed);
        } catch (std::invalid_argument const& err) {
            return;
        } catch (std::out_of_range const& err) {
            return;
        }

        total_processed += processed;

        if (total_processed == line.length()) {
            return;
        }

        str += processed;

        list.push_back(parsed);
    }
}

/**
 * converts an int to a size_t
 */
bool int_to_size_t(int v, std::size_t &to_set) {
    if (v < 0) {
        return false;
    }

    to_set = (std::size_t)((unsigned)v);

    return true;
}

enum class EditKind {
    SUB,
    INS,
    DEL,
    MAT
};

std::ostream& operator<<(std::ostream& out, const EditKind& self) {
    switch (self) {
    case EditKind::MAT:
        out << "m";
        break;
    case EditKind::SUB:
        out << "s";
        break;
    case EditKind::DEL:
        out << "d";
        break;
    case EditKind::INS:
        out << "i";
        break;
    default:
        __builtin_unreachable();
        break;
    }

    return out;
}

struct Edit {
    int cost;
    EditKind kind;

    Edit(int cost, EditKind kind) :
        cost(cost), kind(kind)
    {}
};

std::ostream& operator<<(std::ostream& out, const Edit& self) {
    out << self.cost << self.kind;

    return out;
}

std::vector<std::vector<Edit>> edit_distance(
    const std::string& from,
    const std::string& to,
    int ins,
    int del,
    int sub
) {
    std::vector<std::vector<Edit>> memory;
    std::size_t mem_from_len = from.size() + 1;
    std::size_t mem_to_len = to.size() + 1;

    for (std::size_t to_index = 0; to_index < mem_to_len; ++to_index) {
        std::vector<Edit> row;

        for (std::size_t from_index = 0; from_index < mem_from_len; ++from_index) {
            Edit cell = Edit(0, EditKind::MAT);

            if (to_index == 0 && from_index > 0) {
                cell.cost = row[from_index - 1].cost + del;
                cell.kind = EditKind::DEL;
            } else if (to_index > 0 && from_index == 0) {
                cell.cost = memory[to_index - 1][0].cost + ins;
                cell.kind = EditKind::INS;
            } else if (to_index > 0 && from_index > 0) {
                int curr_sub = memory[to_index - 1][from_index - 1].cost + sub;
                // because this row is not in memory yet we just need to access
                // the current row
                int curr_del = row[from_index - 1].cost + del;
                int curr_ins = memory[to_index - 1][from_index].cost + ins;

                if (from[from_index - 1] == to[to_index - 1]) {
                    cell.cost = memory[to_index - 1][from_index - 1].cost;

                    if (curr_sub < cell.cost) {
                        cell.cost = curr_sub;
                        cell.kind = EditKind::SUB;
                    }
                } else {
                    cell.cost = curr_sub;
                    cell.kind = EditKind::SUB;
                }

                if (curr_del < cell.cost) {
                    cell.cost = curr_del;
                    cell.kind = EditKind::DEL;
                }

                if (curr_ins < cell.cost) {
                    cell.cost = curr_ins;
                    cell.kind = EditKind::INS;
                }
            }

            row.push_back(cell);
        }

        memory.push_back(row);
    }

    return memory;
}

int main(int argc, char *argv[]) {
    std::string line;
    std::vector<int> line_result;
    std::size_t total_lines = 0;
    int ins = 0;
    int del = 0;
    int sub = 0;

    if (std::getline(std::cin, line)) {
        get_int_list(line, line_result);

        if (line_result.size() != 1) {
            std::cout << "total amount of words not specified\n";
            return 1;
        }

        if (line_result[0] <= 0) {
            std::cout << "number of words specified is less or equal to 0\n";
            return 1;
        }

        total_lines = (std::size_t)line_result[0];
    } else {
        std::cout << "failed to read from stdin\n";
        return 1;
    }

    if (std::getline(std::cin, line)) {
        line_result.clear();

        get_int_list(line, line_result);

        if (line_result.size() != 3) {
            std::cout << "you must specify 3 intagers for insert, delete, and substitute\n";
            return 1;
        }

        ins = line_result[0];
        del = line_result[1];
        sub = line_result[2];
    } else {
        std::cout << "failed to read stdin\n";
        return 1;
    }

    for (; std::getline(std::cin, line) && total_lines--;) {
        std::string::size_type space_index = line.find(" ");

        if (space_index == std::string::npos) {
            std::cout << "failed to find space delimiter for word pair: \"" << line << "\"\n";
            return 1;
        }

        std::string from = line.substr(0, space_index);
        std::string to = line.substr(space_index + 1);

        std::vector<std::vector<Edit>> result = edit_distance(from, to, ins, del, sub);

        std::size_t from_index = from.size();
        std::size_t to_index = to.size();
        int edit_value = result[to_index][from_index].cost;

        std::string from_output;
        std::string to_output;

        while (from_index != 0 && to_index != 0) {
            switch (result[to_index][from_index].kind) {
            case EditKind::MAT:
                from_output += from[from_index - 1];
                to_output += to[to_index - 1];

                from_index -= 1;
                to_index -= 1;
                break;
            case EditKind::SUB:
                from_output += from[from_index - 1];
                to_output += to[to_index - 1];

                from_index -= 1;
                to_index -= 1;
                break;
            case EditKind::INS:
                from_output += '_';
                to_output += to[to_index - 1];

                to_index -= 1;
                break;
            case EditKind::DEL:
                from_output += from[from_index - 1];
                to_output += '_';

                from_index -= 1;
                break;
            default:
                __builtin_unreachable();
                break;
            }
        }

        while (from_index != 0) {
            from_output += from[from_index - 1];
            to_output += '_';

            from_index -= 1;
        }

        while (to_index != 0) {
            from_output += '_';
            to_output += to[to_index - 1];

            to_index -= 1;
        }

        std::reverse(from_output.begin(), from_output.end());
        std::reverse(to_output.begin(), to_output.end());

        std::cout << from_output << "\n" << to_output << "\n" << edit_value << "\n";
    }

    return 0;
}
