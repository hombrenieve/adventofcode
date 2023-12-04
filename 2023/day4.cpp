#include "common.h"

struct game {
    std::vector<int> numbers;
    std::vector<int> winning_numbers;

    int get_score() const {
        int score = 0;
        for (const auto n: numbers) {
            if (std::find(winning_numbers.cbegin(), winning_numbers.cend(), n) != winning_numbers.cend()) {
                if(score > 0) {
                    score *= 2;
                } else {
                    score = 1;
                }
            }
        }
        return score;
    }

};


std::vector<game> load_games() {
    std::vector<game> games;
    do_each_input([&](const std::string& line) {
        game g;
        auto tokens = split(line, ":");
        auto numbers = split(tokens[1], "|");
        auto numbers_normal = split_with_rep(numbers[0], ' ');
        std::for_each(numbers_normal.cbegin(), numbers_normal.cend(), [&](const std::string& s) {
            g.numbers.push_back(std::stoi(s));
        });
        auto numbers_winning = split_with_rep(numbers[1], ' ');
        std::for_each(numbers_winning.cbegin(), numbers_winning.cend(), [&](const std::string& s) {
            g.winning_numbers.push_back(std::stoi(s));
        });
        games.push_back(g);
    });
    return games;
}

int main() {
    auto games = load_games();
    int final_score = 0;
    std::for_each(games.cbegin(), games.cend(), [&](const game& g) {
        final_score += g.get_score();
    });
    std::cout << final_score << std::endl;
    return 0;
}