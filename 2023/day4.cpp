#include "common.h"

struct game {
    std::vector<int> numbers;
    std::vector<int> winning_numbers;

    int get_score() const {
        int score = 0;
        for (const auto n: numbers) {
            if (std::find(winning_numbers.cbegin(), winning_numbers.cend(), n) != winning_numbers.cend()) {
                score++;
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
    std::vector<int> scores(games.size(), 1);
    for(int i = 0; i < games.size(); i++) {
        const auto gs = games[i].get_score();
        for(int j = i + 1; j <= i + gs; j++) {
            scores[j] += scores[i];
        }
    }
    std::cout << std::accumulate(scores.begin(), scores.end(), 0) << std::endl;
    return 0;
}