#include "common.h"

struct range {
    long source_min;
    long dest_min;
    long steps;
};

struct mapping {
    std::string name;
    std::vector<range> ranges;

    long map(long source) const {
        for(const auto& r: ranges) {
            if(source >= r.source_min && source < r.source_min + r.steps) {
                return r.dest_min + (source - r.source_min);
            }
        }
        return source;
    }
};


struct seed {
    long value;
    long soil;
    long fertilizer;
    long water;
    long light;
    long temperature;
    long humidity;
    long location;

    seed(long s) : value(s), soil(-1), fertilizer(-1), water(-1), light(-1), temperature(-1), humidity(-1), location(-1) {}
    
    void fill_up(const std::vector<mapping>& mappings) {
        //ORDER IS IMPORTANT
        soil = mappings[0].map(value);
        fertilizer = mappings[1].map(soil);
        water = mappings[2].map(fertilizer);
        light = mappings[3].map(water);
        temperature = mappings[4].map(light);
        humidity = mappings[5].map(temperature);
        location = mappings[6].map(humidity);
    }
};




std::vector<seed> load_seeds() {
    std::vector<seed> seeds;
    do_each_input_until_empty([&](const std::string& line) {
        auto tokens = split(line, ":");
        auto numbers = split_with_rep(tokens[1], ' ');
        std::for_each(numbers.cbegin(), numbers.cend(), [&](const std::string& s) {
            seeds.emplace_back(std::stol(s));
        });
    });
    return seeds;
}

std::vector<mapping> load_mappings() {
    std::vector<mapping> mappings;
    mapping m;
    do_each_input([&](const std::string& line) {
        if(line.empty()) {
            mappings.push_back(m);
            m = mapping();
            return;
        }
        if(m.name.empty()) {
            auto tokens = split(line, " ");
            m.name = tokens[0];
            return;
        }
        auto numbers = split_with_rep(line, ' ');
        m.ranges.emplace_back(std::stol(numbers[1]), std::stol(numbers[0]), std::stol(numbers[2]));
    });
    return mappings;
}

int main() {
    auto seeds = load_seeds();
    auto mappings = load_mappings();
    std::for_each(seeds.begin(), seeds.end(), [&](seed& s) {
        s.fill_up(mappings);
    });
    auto min = std::min_element(seeds.begin(), seeds.end(), [](const seed& s1, const seed& s2) {
        return s1.location < s2.location;
    });
    std::cout << min->location << std::endl;
    return 0;
}