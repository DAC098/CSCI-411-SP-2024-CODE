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

struct HotelCost {
    int cost;
    std::size_t hotel;

    HotelCost(int cost, std::size_t hotel) :
        cost(cost), hotel(hotel)
    {}
};

int sqd(int v) {
    return v * v;
}

std::vector<HotelCost> hotel_distances(const std::vector<int>& hotels, int travel) {
    std::vector<HotelCost> memory;
    memory.reserve(hotels.size());
    memory.push_back(HotelCost(0, 0));

    for (std::size_t memory_index = 1; memory_index < hotels.size(); ++memory_index) {
        HotelCost marker(sqd(travel - hotels[memory_index]), memory_index);

        for (std::size_t hotel_index = 0; hotel_index < memory_index; ++hotel_index) {
            int penalty = sqd(travel - (hotels[memory_index] - hotels[hotel_index]));
            int prev = memory[hotel_index].cost + penalty;

            if (prev < marker.cost) {
                marker.cost = prev;
                marker.hotel = hotel_index;
            }
        }

        memory.push_back(marker);
    }

    return memory;
}

int main(int argc, char *argv[]) {
    std::string line;
    std::vector<int> line_result;
    std::vector<int> hotels;
    int travel = 0;
    int expected = 0;

    std::cout << "Enter the number of hotels and the ideal number of miles to travel per day: ";

    if (std::getline(std::cin, line)) {
        get_int_list(line, line_result);

        if (line_result.size() != 2) {
            std::cout << "hotels and number of miles traveled is required\n";
            return 1;
        }

        if (line_result[0] <= 0) {
            std::cout << "number of hotels specified is less or equal to 0\n";
            return 1;
        }

        if (line_result[1] <= 0) {
            std::cout << "travel distance specified is less or equal to 0\n";
            return 1;
        }

        expected = line_result[0];
        travel = line_result[1];

        hotels.reserve((std::size_t)expected + 1);
    } else {
        std::cout << "failed to read from stdin\n";
        return 1;
    }

    std::cout << "Enter " << expected << " hotel distances each on a separate line: ";

    hotels.push_back(0);

    for (; std::getline(std::cin, line) && expected--;) {
        line_result.clear();

        get_int_list(line, line_result);

        if (line_result.size() != 1) {
            std::cout << "hotel distance not specified\n";
            return 1;
        }

        hotels.push_back(line_result[0]);
    }

    std::vector<HotelCost> result = hotel_distances(hotels, travel);

    std::size_t next_index = result.size() - 1;
    std::vector<std::size_t> visited;

    while (next_index != 0) {
        visited.push_back(next_index);

        if (result[next_index].hotel == next_index) {
            break;
        } else {
            next_index = result[next_index].hotel;
        }
    }

    std::cout << "Hotels to visit:";

    std::vector<size_t>::reverse_iterator iter = visited.rbegin();
    std::vector<size_t>::reverse_iterator end = visited.rend();

    for (; iter != end; ++iter) {
        std::cout << " " << *iter;
    }

    std::cout << " \nTotal penalty: " << result[result.size() - 1].cost << "\n";
}
