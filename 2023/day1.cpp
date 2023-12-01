#include "common.h"
#include <numeric>

std::vector<int> load_digits() {
    std::vector<int> digits;
    do_each_input([&](const std::string &line) {
        std::string numbers = "";
        for(char c : line) {
            if(isdigit(c)) {
                numbers += c;
            }
        }
        std::string finalnumber = numbers.substr(0,1) + numbers.substr(numbers.size()-1);
        digits.push_back(std::stoi(finalnumber));
    });
    return digits;
}

int main() {
    std::vector<int> digits = load_digits();
    for(int i : digits) {
        std::cout << i << std::endl;
    }
    std::cout << "Sum: " << std::accumulate(digits.begin(), digits.end(), 0) << std::endl;
    return 0;
}