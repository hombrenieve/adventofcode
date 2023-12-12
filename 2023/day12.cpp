#include "common.h"
#include <boost/regex.hpp>


class condition_record {
    std::string template_arrangement;
    std::vector<int> registers;
    boost::regex regexp;

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
        boost::regex r(regexp);
        boost::cmatch m;
        int count = 0;
        for(const auto& arrangement : arrangements) {
            if(boost::regex_match(arrangement.c_str(), m, r)) {
                count++;
            }
        }
        return count;
    }

    bool possible_match(const std::string& partial_arrangement) const {
        boost::match_results<std::string::const_iterator> what;
        auto matched = boost::regex_match(partial_arrangement, what, regexp, 
            boost::match_default | boost::match_partial);
        return matched;
    }

    bool complete_match(const std::string& arrangement) const {
        boost::cmatch what;
        return boost::regex_match(arrangement.c_str(), what, regexp);
    }

    std::vector<std::string> get_possible_arrangements(const std::string& ctemplate, const std::string& sofar) {
        std::vector<std::string> arrangements;
        if(ctemplate.size() == 0) {
            return {};
        }
        if(ctemplate.size() == 1) {
            if(ctemplate[0] == '?') {
                if(possible_match(sofar + ".")) {
                    arrangements.push_back(".");
                }
                if(possible_match(sofar + "#")) {
                    arrangements.push_back("#");
                }
            } else {
                arrangements.push_back(std::string(1, ctemplate[0]));
            }
        } else {
            if(ctemplate[0] == '?') {
                if(possible_match(sofar + ".")) {
                    auto sub_arrangements = get_possible_arrangements(ctemplate.substr(1), sofar + ".");
                    for(const auto& sub_arrangement : sub_arrangements) {
                        //add it if possible
                        if(complete_match(sofar + "." + sub_arrangement)) {
                            arrangements.push_back("." + sub_arrangement);
                        }
                    }
                }
                if(possible_match(sofar + "#")) {
                    auto sub_arrangements = get_possible_arrangements(ctemplate.substr(1), sofar + "#");
                    for(const auto& sub_arrangement : sub_arrangements) {
                        //add it if possible
                        if(complete_match(sofar + "#" + sub_arrangement)) {
                            arrangements.push_back("#" + sub_arrangement);
                        }
                    }
                }
            } else {
                auto sub_arrangements = get_possible_arrangements(ctemplate.substr(1), sofar + std::string(1, ctemplate[0]));
                for(const auto& sub_arrangement : sub_arrangements) {
                    //add it if possible
                    if(complete_match(sofar + std::string(1, ctemplate[0]) + sub_arrangement)) {
                        arrangements.push_back(std::string(1, ctemplate[0]) + sub_arrangement);
                    }
                }
            }
        }
        return arrangements;
    }

public:
    condition_record(const std::string& arrangement, const std::vector<int>& registers) : template_arrangement(arrangement), registers(registers) {
        std::string regexps;
        for(int i = 0; i < registers.size(); i++) {
            if(i == 0) {
                regexps += std::string("(\\.*)");
            }
            regexps += std::string("#{") + std::to_string(registers[i]) + "}";
            if(i != registers.size() - 1) {
                regexps += std::string("(\\.+)");
            } else {
                regexps += std::string("(\\.*)");
            }
        }
        regexp = boost::regex(regexps);
    }

    int possible_arrangments() {
        auto pa = get_possible_arrangements(template_arrangement, "");
        return pa.size();
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