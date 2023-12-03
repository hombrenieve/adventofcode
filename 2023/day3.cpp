#include "common.h"

std::vector<std::string> load_schematic() {
    std::vector<std::string> schematic;
    do_each_input([&](const std::string& line) {
        schematic.push_back(line);
    });
    return schematic;
}

bool is_symbol_adjacent_to(const std::vector<std::string>& schematic, int line, int starting, int ending) {
    for(int l = line - 1; l <= line + 1; l++) {
        if(l < 0 || l >= schematic.size()) continue;
        for(int i = starting - 1; i <= ending; i++) {
            if(i < 0 || i >= schematic[l].size()) continue;
            if(schematic[l][i] != '.' && !isdigit(schematic[l][i])) return true;
        }
    }
    return false;
}

std::vector<int> get_part_numbers(const std::vector<std::string>& schematic) {
    std::vector<int> part_numbers;
    for(int i = 0; i < schematic.size(); i++) {
        int is_number = -1;
        for(int j = 0; j < schematic[i].size(); j++) {
            if(isdigit(schematic[i][j]) && is_number == -1) {
                is_number = j;
            } else if (!isdigit(schematic[i][j]) && is_number != -1) {
                //check if there is a symbol adjacent to the number
                if (is_symbol_adjacent_to(schematic, i, is_number, j)) {
                    //push the number
                    auto number = schematic[i].substr(is_number, j - is_number);
                    part_numbers.push_back(std::stoi(number));
                } else {
                    std::cout << "NOT A PART NUMBER: " << schematic[i].substr(is_number, j - is_number) << std::endl;
                }
                is_number = -1;
            }
        }
        if(is_number != -1) {
            //check if there is a symbol adjacent to the number
            if (is_symbol_adjacent_to(schematic, i, is_number, schematic[i].size())) {
                //push the number
                auto number = schematic[i].substr(is_number, schematic[i].size() - is_number);
                part_numbers.push_back(std::stoi(number));
            } else {
                std::cout << "NOT A PART NUMBER: " << schematic[i].substr(is_number, schematic[i].size() - is_number) << std::endl;
            }
        }
    }
    return part_numbers;
}

int build_number_from_pos(const std::vector<std::string>& schematic, int line, int pos) {
    //First go backwards to find starting position
    int starting = pos;
    while(starting >= 0 && isdigit(schematic[line][starting])) {
        starting--;
    }
    if(!isdigit(schematic[line][starting])) {
        starting++;
    }
    auto ending = pos;
    while(ending < schematic[line].size() && isdigit(schematic[line][ending])) {
        ending++;
    }
    if(ending == schematic[line].size()) {
        ending++;
    }
    std::cout << "Starting: " << starting << ", Ending: " << ending << " Number: " << schematic[line].substr(starting, ending - starting) << std::endl;
    return std::stoi(schematic[line].substr(starting, ending - starting));
}

std::vector<int> get_gear_ratios(const std::vector<std::string>& schematic) {
    std::vector<int> gear_ratios;
    for(int i = 0; i < schematic.size(); i++) {
        int is_number = -1;
        for(int j = 0; j < schematic[i].size(); j++) {
            std::vector<int> numbers;
            if(schematic[i][j] == '*') {
                std::cout << "Found a gear at " << i << ", " << j << std::endl;
                //check adyacent postions for numbers
                for(int line = i - 1; line <= i + 1; line++) {
                    if(line < 0 || line >= schematic.size()) continue;
                    int found_number = -1;
                    for(int k = j - 1; k <= j + 1; k++) {
                        if(k < 0 || k >= schematic[line].size()) continue;
                        if(isdigit(schematic[line][k])) {
                            found_number = k;
                        } else if(found_number != -1) {
                            //build number
                            std::cout << "Found at:" << line << ", " << k-1 << std::endl;
                            auto number = build_number_from_pos(schematic, line, k-1);
                            std::cout << "Found number: " << number << std::endl;
                            if(number != -1) {
                                numbers.push_back(number);
                            }
                            found_number = -1;
                            continue;
                        }
                    }
                    if(found_number != -1) {
                        //build number
                        std::cout << "Found at:" << line << ", " << found_number << std::endl;
                        auto number = build_number_from_pos(schematic, line, found_number);
                        std::cout << "Found number: " << number << std::endl;
                        if(number != -1) {
                            numbers.push_back(number);
                        }
                    }
                }
                if(numbers.size() == 2) {
                    gear_ratios.push_back(numbers[0] * numbers[1]);
                }
            }
        }
    }
    return gear_ratios;
}

int main() {
    auto schematic = load_schematic();
    auto gear_ratios = get_gear_ratios(schematic);
    std::cout << "Part 2: " << std::accumulate(gear_ratios.begin(), gear_ratios.end(), 0) << std::endl;
    return 0;
}