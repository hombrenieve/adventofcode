#include "common.h"
#include <regex>


class condition_record {
    std::string template_arrangement;
    std::vector<int> registers;
    std::string regexp;

    constexpr std::vector<std::string> get_all_arrangements(const std::string& template_arrangement) {
        std::vector<std::string> arrangements;
        if(template_arrangement.size() == 0) {
            return {};
        }
        if(template_arrangement.size() == 1) {
            if(template_arrangement[0] == '?') {
                arrangements = {".", "#"};
            } else {
                arrangements.push_back(std::string(1, template_arrangement[0]));
            }
        } else {
            if(template_arrangement[0] == '?') {
                auto sub_arrangements = get_all_arrangements(template_arrangement.substr(1));
                for(const auto& sub_arrangement : sub_arrangements) {
                    arrangements.push_back("." + sub_arrangement);
                    arrangements.push_back("#" + sub_arrangement);
                }
            } else {
                auto sub_arrangements = get_all_arrangements(template_arrangement.substr(1));
                for(const auto& sub_arrangement : sub_arrangements) {
                    arrangements.push_back(std::string(1, template_arrangement[0]) + sub_arrangement);
                }
            }
        }
        return arrangements;
    }

    int arrangement_matches(const std::vector<std::string>& arrangements) const {
        std::regex r(regexp);
        std::smatch m;
        int count = 0;
        for(const auto& arrangement : arrangements) {
            if(std::regex_match(arrangement, m, r)) {
                count++;
            }
        }
        return count;
    }

public:
    condition_record(const std::string& arrangement, const std::vector<int>& registers) : template_arrangement(arrangement), registers(registers) {
        for(int i = 0; i < registers.size(); i++) {
            if(i == 0) {
                regexp += std::string("(\\.*)");
            }
            regexp += std::string("#{") + std::to_string(registers[i]) + "}";
            if(i != registers.size() - 1) {
                regexp += std::string("(\\.+)");
            } else {
                regexp += std::string("(\\.*)");
            }
        }
    }

    int possible_arrangments() {
        return arrangement_matches(get_all_arrangements(template_arrangement));
    }
};

std::vector<condition_record> load_records() {
    std::vector<condition_record> records;
    do_each_input([&](const std::string& line) {
        std::vector<std::string> splitted = split(line, " ");
        auto arrangement = splitted[0];
        auto registers = split_with_rep(splitted[1], ',');
        std::vector<int> registers_int;
        for (const auto& reg : registers) {
            registers_int.push_back(std::stoi(reg));
        }
        records.emplace_back(arrangement, registers_int);
    });
    return records;
}

struct day12 {
    std::vector<condition_record> records;

    day12() : records(load_records()) {}
    
    int part1() {
        return std::accumulate(records.begin(), records.end(), 0, [&](int sum, condition_record& record) {
            return sum + record.possible_arrangments();
        });
    }
};

int main() {
    day12 d;
    std::cout << d.part1() << std::endl;
    return 0;
}