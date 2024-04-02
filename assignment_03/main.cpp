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

struct Change {
    bool is_set = false;
    std::size_t count = 0;
    std::size_t last_used = 0;
    std::size_t possible = 0;
};

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

/**
 * calculates all values in mem with the given denominations
 */
void calc_bottom_up(std::vector<std::size_t> dnmn, std::vector<Change> &mem) {
    mem[0].is_set = true;
    mem[0].count = 0;
    mem[0].last_used = 0;

    bool is_set;
    std::size_t count;
    std::size_t last_used;
    std::size_t cmp;
    std::size_t dnmn_index;

    for (std::size_t index = 1; index < mem.size(); ++index) {
        is_set = false;
        count = 0;
        last_used = 0;

        for (dnmn_index = 0; dnmn_index < dnmn.size(); ++dnmn_index) {
            if (dnmn[dnmn_index] > index) {
                continue;
            }

            if (!mem[index - dnmn[dnmn_index]].is_set) {
                continue;
            }

            cmp = mem[index - dnmn[dnmn_index]].count + 1;

            if (is_set) {
                if (count > cmp) {
                    last_used = dnmn_index;
                    count = cmp;
                }
            } else {
                last_used = dnmn_index;
                count = cmp;
                is_set = true;
            }
        }

        mem[index].is_set = is_set;
        mem[index].count = count;
        mem[index].last_used = last_used;
    }
}

int main(int argc, char *argv[]) {
    std::string line;
    std::vector<int> line_result;
    std::vector<std::size_t> denominations;
    std::vector<std::size_t> checks;
    std::size_t denomination_count;
    std::size_t check_count;
    std::size_t conv;
    std::size_t max_size = 0;

    if (std::getline(std::cin, line)) {
        get_int_list(line, line_result);

        if (line_result.size() != 2) {
            std::cout << "invalid change line provided: \"" << line << "\"\n";
            return 1;
        }

        if (!int_to_size_t(line_result[0], denomination_count)) {
            std::cout << "amount of denominations specified is invalid: " << line_result[0] << "\n";
            return 1;
        }

        if (!int_to_size_t(line_result[1], check_count)) {
            std::cout << "amount of checks specified is invalid: " << line_result[1] << "\n";
            return 1;
        }
    }

    if (std::getline(std::cin, line)) {
        line_result.clear();
        get_int_list(line, line_result);

        if (line_result.size() == 0) {
            std::cout << "no denominations specified\n";
            return 1;
        }

        for (int v : line_result) {
            if (!int_to_size_t(v, conv)) {
                std::cout << "invalid denominations line provided: \"" << line << "\"\n";
                return 1;
            }

            denominations.push_back(conv);
        }

        if (denominations.size() != denomination_count) {
            std::cout << "denominations provided does not match the specified amount\n";
            return 1;
        }
    }

    for (; std::getline(std::cin, line);) {
        line_result.clear();

        get_int_list(line, line_result);

        if (line_result.size() == 0) {
            std::cout << "no check amount was specified\n";
            return 1;
        }

        if (!int_to_size_t(line_result[0], conv)) {
            std::cout << "invalid check value provided: \"" << line << "\"\n";
            return 1;
        }

        if (conv > max_size) {
            max_size = conv;
        }

        checks.push_back(conv);
    }

    if (checks.size() != check_count) {
        std::cout << "checks provided does not match the specified amount\n";
        return 1;
    }

    max_size += 1;

    std::size_t index;
    std::size_t running_index;
    Change empty;
    std::vector<std::size_t> running(denominations.size());
    std::vector<Change> memorized(max_size, empty);

    calc_bottom_up(denominations, memorized);

    for (std::size_t value : checks) {
        index = value;

        while (true) {
            if (!memorized[index].is_set) {
                break;
            }

            if (memorized[index].count == 0) {
                break;
            }

            running[memorized[index].last_used] += 1;
            index = index - denominations[memorized[index].last_used];
        }

        for (running_index = 0; running_index < running.size(); ++running_index) {
            std::cout << running[running_index] << " ";
            running[running_index] = 0;
        }

        std::cout << "\n";
    }

    return 0;
}
