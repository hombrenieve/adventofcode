#include "common.h"
#include <climits>


struct interval {
    long min;
    long max;

    interval(long min, long steps) : min(min), max(min+steps) {}

    long size() const {
        return max - min;
    }

    bool contains(long value) const {
        return value >= min && value < max;
    }

    std::vector<interval> intersection(const interval& other) const {
        if(other.min > max || other.max < min) {
            return {};
        }
        if(other.min >= min && other.max <= max) {
            return {other};
        }
        if(other.min < min && other.max > max) {
            return {*this};
        }
        if(other.min <= min) {
            return {interval(min, other.max - min)};
        }
        return {interval(other.min, max - other.min)};
    }

    std::vector<interval> difference(const interval& other) const {
        if(other.min > max || other.max < min) {
            return {*this};
        }
        if(other.min > min && other.max < max) {
            return {interval(min, other.min - min), interval(other.max, max - other.max)};
        }
        if(other.min <= min && other.max >= max) {
            return {};
        }
        if(other.min < min) {
            return {interval(other.max, max - other.max)};
        }
        return {interval(min, other.min - min)};
    }
};

std::ostream& operator<<(std::ostream& os, const std::vector<interval>& intervals) {
    os << "{";
    for(const auto& i: intervals) {
        os << "[" << i.min << ", " << i.max << "], ";
    }
    os << "}";
    return os;
}

struct range_map {
    interval source;
    interval dest;

    range_map(long source_min, long dest_min, long steps) : source(source_min, steps), dest(dest_min, steps) {}

    std::pair<std::vector<interval>, std::vector<interval>> map(const interval& source) const {
        auto intersections = source.intersection(this->source);
        auto differences = source.difference(this->source);
        std::transform(std::begin(intersections), std::end(intersections), intersections.begin(), [&](const interval& i) {
            return interval(dest.min + (i.min - this->source.min), i.size());
        });
        //returns transformed and left sources
        return {intersections, differences};
    }
};

struct mapping {
    std::string name;
    std::vector<range_map> ranges;

    long map(long source) const {
        for(const auto& r: ranges) {
            if(r.source.contains(source)) {
                return r.dest.min + (source - r.source.min);
            }
        }
        return source;
    }

    std::vector<interval> map(const interval& source) const {
        std::vector<interval> result;
        std::vector<interval> next_sources{source};
        std::vector<interval> sources{source};
        for(const auto& r: ranges) {
            for(const auto& s: sources) {
                auto [intersections, differences] = r.map(s);
                result.insert(std::end(result), std::begin(intersections), std::end(intersections));
                std::for_each(intersections.begin(), intersections.end(), [&](const interval& i) {
                    result.push_back(i);
                });
                std::for_each(differences.begin(), differences.end(), [&](const interval& i) {
                    next_sources.push_back(i);
                });
            }
            sources = next_sources;
        }
        std::for_each(sources.begin(), sources.end(), [&](const interval& i) {
            result.push_back(i);
        });
        return result;
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




std::vector<interval> load_seeds() {
    std::vector<interval> seeds;
    do_each_input_until_empty([&](const std::string& line) {
        auto tokens = split(line, ":");
        auto numbers = split_with_rep(tokens[1], ' ');
        for(int i = 0; i < numbers.size(); i+=2) {
            seeds.emplace_back(std::stol(numbers[i]), std::stol(numbers[i + 1]));
        }
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

long find_min(interval& data, const std::vector<mapping>& mappings) {
    std::vector<interval> result{data};
    for(const auto& m: mappings) {
        for(const auto& i: result) {
            result = m.map(i);
        }
        std::cout << "Mapping " << m.name << " -> " << result << std::endl;
    }
    return std::min_element(result.begin(), result.end(), [](const interval& a, const interval& b) {
        return a.min < b.min;
    })->min;
}

int main() {
    auto seeds = load_seeds();
    auto mappings = load_mappings();
    std::vector<long> mins;
    for(auto& s: seeds) {
        mins.push_back(find_min(s, mappings));
    }
    std::cout << "Min: " << *std::min_element(mins.begin(), mins.end()) << std::endl;
    return 0;
}