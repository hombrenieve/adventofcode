#include "common.h"

struct race {
    int time;
    int distance;
};

std::vector<race> load_races() {
    std::vector<race> result;
    do_each_input([&](const std::string& line) {
        auto line_split = split_with_rep(line, ' ');
        if(line_split[0] == "Time:") {
            for(int i = 1; i < line_split.size(); i++) {
                result.push_back({std::stoi(line_split[i]), 0});
            }
        } else {
            for(int i = 1; i < line_split.size(); i++) {
                result[i-1].distance = std::stoi(line_split[i]);
            }
        }
    });
    return result;
}

constexpr int get_distance(int time_holding, int total_time) {
    return time_holding * (total_time - time_holding);
}

int calculate_choices(const race& r) {
    int choices = 0;
    for(int i = 0; i < r.time; i++) {
        if(get_distance(i, r.time) > r.distance) {
            choices++;
        }
    }
    return choices;
}

int main() {
    auto races = load_races();
    int result = 1;
    for(const auto& r: races) {
        auto choices = calculate_choices(r);
        std::cout << choices << std::endl;
        result *= choices;
    }
    std::cout << result << std::endl;
}