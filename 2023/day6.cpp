#include "common.h"

struct race {
    long time;
    long distance;
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

constexpr int get_distance(int time_holding, int total_time) {
    return time_holding * (total_time - time_holding);
}

long find_half(const race& r, long half_size) {
    std::cout << "Calculating half_size: " << half_size << std::endl;
    if(half_size >= r.time/2) {
        return -1;
    }
    if(get_distance(r.time/2 - half_size, r.time) > r.distance && get_distance(r.time/2 + half_size, r.time) > r.distance) {
        long calc = find_half(r, half_size*2);
        if(calc == -1) {
            return half_size;
        } else {
            return calc;
        }
    } else {
        return -1;
    }
}

long find_min(const race&r, long half_size) {
    if(half_size >= r.time/2) {
        return 0;
    }
    if(get_distance(r.time/2 - half_size, r.time) > r.distance) {
        //go backwards
        while(get_distance(r.time/2 - half_size, r.time) > r.distance) {
            half_size--;
        }
        return half_size;
    } else {
        //go forwards
        while(get_distance(r.time/2 - half_size, r.time) > r.distance) {
            half_size++;
        }
        return half_size;
    }
}

long find_max(const race&r, long half_size) {
    if(half_size >= r.time/2) {
        return r.time/2;
    }
    if(get_distance(r.time/2 + half_size - 1, r.time) > r.distance) {
        //go forwards
        while(get_distance(r.time/2 + half_size, r.time) > r.distance) {
            half_size++;
        }
        return half_size;
    } else {
        //go backwards
        while(get_distance(r.time/2 + half_size, r.time) > r.distance) {
            half_size--;
        }
        return half_size;
    }
}

long find_min_max(const race& r) {
    std::cout << "time: " << r.time << std::endl;
    auto half = find_half(r, 1);
    std::cout << "half: " << half << std::endl;
    auto min = find_min(r, half);
    auto max = find_max(r, half);
    std::cout << "min: " << min << std::endl;
    std::cout << "max: " << max << std::endl;
    return max - min -1;
}

int main() {
    auto race = load_race();
    auto choices = find_min_max(race);
    std::cout <<  choices << std::endl;
}