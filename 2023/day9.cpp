#include "common.h"


std::vector<std::vector<int>> load_sequences() {
    std::vector<std::vector<int>> sequences;
    do_each_input([&sequences](std::string line) {
        std::vector<int> sequence;
        auto str = split_with_rep(line, ' ');
        std::for_each(str.begin(), str.end(), [&sequence](const std::string &s) {
            sequence.push_back(std::stoi(s));
        });
        sequences.push_back(sequence);
    });
    return sequences;
}



int extrapolate(const std::vector<int> &sequence) {
    if(meet_condition_all<int>(sequence, [](const int &v) { return v == 0; })) {
        return 0;
    }
    //calculate next level
    std::vector<int> differences;
    for(int i = 0; i < sequence.size()-1; i++) {
        differences.push_back(sequence[i + 1] - sequence[i]);
    }
    int next_level = extrapolate(differences);
    //calculate last number
    return sequence[0] - next_level;
}

int main() {
    auto sequences = load_sequences();
    std::vector<int> results;
    for(const auto &sequence : sequences) {
        results.push_back(extrapolate(sequence));
    }
    std::cout << "Results: " << std::accumulate(results.begin(), results.end(), 0) << std::endl;
    return 0;
}