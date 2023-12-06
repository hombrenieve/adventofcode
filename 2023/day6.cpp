#include "common.h"
#include <cmath>

struct race {
    long long time;
    long long distance;

    long long get_max() const {
        return floor((time*1.0 + std::sqrt(time*time*1.0 - 4.0*distance))/2.0);
    }

    long long get_min() const {
        return round((time*1.0 - std::sqrt(time*time*1.0 - 4.0*distance))/2.0);
    }
};

race load_race() {
    race result;
    do_each_input([&](const std::string& line) {
        auto line_split = split_with_rep(line, ' ');
        if(line_split[0] == "Time:") {
            std::string time_str = line_split[1];
            for(int i = 2; i < line_split.size(); i++) {
                time_str += line_split[i];
            }
            result.time = std::stol(time_str);
        } else {
            std::string distance_str = line_split[1];
            for(int i = 2; i < line_split.size(); i++) {
                distance_str += line_split[i];
            }
            result.distance = std::stol(distance_str);
        }
    });
    return result;
}


int main() {
    auto race = load_race();
    std::cout << "Min is " << race.get_min() << " Max is: " << race.get_max() << std::endl;
    std::cout <<  "Choices " << race.get_max()-race.get_min() << std::endl;
}