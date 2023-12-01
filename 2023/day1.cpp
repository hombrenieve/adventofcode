#include "common.h"
#include <numeric>
#include <iterator>
#include <cstring>

int get_number_from_pos(int i, const std::string& line) {
    const static std::vector<const char*> numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    if(isdigit(line[i])) {
        return line[i] - '0';
    }
    for(int j = 0; j < 9; j++) {
        if(line.substr(i, strlen(numbers[j])) == std::string(numbers[j])) {
            return j+1;
        }
    }
    return -1;
}

std::vector<int> load_digits() {
    std::vector<int> digits;
    do_each_input([&](const std::string &line) {
        std::string numbers = "";
        for(int i = 0; i < line.size(); i++) {
            int number = get_number_from_pos(i, line);
            if(number != -1) {
                numbers += std::to_string(number);
            }
        }
        std::string finalnumber = numbers.substr(0,1) + numbers.substr(numbers.size()-1);
        std::cout << "Found: " << numbers << " in " << line << " -> " << finalnumber << std::endl;
        digits.push_back(std::stoi(finalnumber));
    });
    return digits;
}

int main() {
    std::vector<int> digits = load_digits();
    std::cout << "Sum: " << std::accumulate(digits.begin(), digits.end(), 0) << std::endl;
    return 0;
}