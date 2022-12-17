// solution to https://adventofcode.com/2022/day/3 (part 2)

#include <iostream>
#include <fstream>
#include <algorithm>
#include <vector>
//#include <pair>
#include <set>

int main (int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "usage: " << argv[0] << " <path to input data>" << std::endl;
        return 1;
    }

    const char* file_path = argv[1];
    std::ifstream file(file_path, std::ios::in);
    if (!file.is_open()) {
        std::cerr << "failed to open file: \"" << file_path << "\"" << std::endl;
        return 1;
    }

    // convert the data file into a vector of pairs of sets (set = compartment of rucksack)
    std::string line;
    std::vector<std::pair<std::set<char>, std::set<char>>> entries;
    while (file >> line) {
        // pivot is the center of the string
        auto pivot = line.begin() + (line.size() / 2);
        entries.insert(entries.end(), {
            { line.begin(), pivot },
            { pivot, line.end() }
        });
    }

    // iterate over these as groups of 3
    if (entries.size() % 3 != 0) {
        std::cerr << "entries in data file must be a multiple of 3" << std::endl;
        return 1; 
    }

    // iterate over all sets of 3 rucksacks
    int total = 0;
    for (auto it = entries.begin(); it != entries.end(); it += 3) {
        auto g1 = *it;
        auto g2 = *(it + 1);
        auto g3 = *(it + 2);

        std::set<char> g1_union;
        std::set<char> g2_union;
        std::set<char> g3_union;

        std::set_union(g1.first.begin(), g1.first.end(), g1.second.begin(), g1.second.end(), std::inserter(g1_union, g1_union.begin()));
        std::set_union(g2.first.begin(), g2.first.end(), g2.second.begin(), g2.second.end(), std::inserter(g2_union, g2_union.begin()));
        std::set_union(g3.first.begin(), g3.first.end(), g3.second.begin(), g3.second.end(), std::inserter(g3_union, g3_union.begin()));

        std::set<char> g1g2_intersection;
        std::set<char> final_intersection;

        std::set_intersection(g1_union.begin(), g1_union.end(), g2_union.begin(), g2_union.end(), std::inserter(g1g2_intersection, g1g2_intersection.begin()));
        std::set_intersection(g1g2_intersection.begin(), g1g2_intersection.end(), g3_union.begin(), g3_union.end(), std::inserter(final_intersection, final_intersection.begin()));

        char badge = *final_intersection.begin();

        if (badge >= 'a' && badge <= 'z') {
            total += (badge - 'a') + 1;
        } else if (badge >= 'A' && badge <= 'Z') {
            total += (badge - 'A') + 27;
        }
    }

    std::cout << "total score: " << total << std::endl;
    return 0;
}