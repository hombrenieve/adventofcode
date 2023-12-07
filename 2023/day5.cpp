#include "common.h"
#include <climits>


struct interval {
    long min;
    long max;

    interval(long min, long steps) : min(min), max(min+steps) {}

    long size() const {
        return max - min;
    }
    
    bool contains(const interval& other) const {
        return min < other.min && max < other.max;
    }

};

std::ostream& operator<<(std::ostream& os, const interval& i) {
    os << "[" << i.min << ", " << i.max << "]";
    return os;
}

std::ostream& operator<<(std::ostream& os, const std::vector<interval>& intervals) {
    os << "{";
    for(const auto& i: intervals) {
        os << i << ", ";
    }
    os << "}";
    return os;
}

struct range_map {
    interval source;
    interval dest;

    range_map(long source_min, long dest_min, long steps) : source(source_min, steps), dest(dest_min, steps) {}

    void apply(std::vector<interval>& sources, std::vector<interval>& mapped) const {
        std::vector<interval> new_sources;
        std::vector<interval> mappeable;
        for(const auto& s: sources) {
            if(s.min > source.max || s.max < source.min) {
                new_sources.push_back(s);
                continue;
            }
            interval i_mappeable(s.min, s.size());
            if(s.min < source.min) {
                new_sources.emplace_back(s.min, source.min - s.min);
                i_mappeable.min = source.min;
            }
            if(s.max > source.max) {
                new_sources.emplace_back(source.max + 1, s.max - source.max - 1);
                i_mappeable.max = source.max;
            }
            mappeable.push_back(i_mappeable);
        }
        for(const auto& m: mappeable) {
            mapped.emplace_back(m.min - source.min + dest.min, m.size());
        }
        sources = new_sources;
    }

};

struct mapping {
    std::string name;
    std::vector<range_map> ranges;

    std::vector<interval> apply(const std::vector<interval>& intervals) const {
        std::vector<interval> sources = intervals;
        std::vector<interval> mapped;
        for(const auto& r: ranges) {
            r.apply(sources, mapped);
        }
        mapped.insert(mapped.end(), sources.begin(), sources.end());
        return mapped;
    }
    
};

std::vector<interval> load_seeds() {
    std::vector<interval> seeds;
    do_each_input_until_empty([&](const std::string& line) {
        auto tokens = split(line, ":");
        auto numbers = split_with_rep(tokens[1], ' ');
        for(int i = 0; i < numbers.size(); i+=2) {
            seeds.emplace_back(std::stol(numbers[i]), std::stol(numbers[i + 1])-1);
        }
    });
    return seeds;
}

//WARNING!!! INPUT NEEDS ONE EMPTY LINE AT THE END
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
        m.ranges.emplace_back(std::stol(numbers[1]), std::stol(numbers[0]), std::stol(numbers[2])-1);
    });
    return mappings;
}

long find_min(const std::vector<interval>& seeds, const std::vector<mapping>& mappings) {
    std::vector<interval> intervals = seeds;
    for(const auto& m: mappings) {
        intervals = m.apply(intervals);
    }
    return std::min_element(intervals.begin(), intervals.end(), [](const interval& a, const interval& b) {
        return a.min < b.min;
    })->min;
}

int main() {
    auto seeds = load_seeds();
    auto mappings = load_mappings();
    auto min = find_min(seeds, mappings);
    std::cout << min << std::endl;
    return 0;
}