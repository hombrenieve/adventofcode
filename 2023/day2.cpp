#include "common.h"

struct bag {
    int red;
    int green;
    int blue;
    
    bag(int red_, int green_, int blue_) : red(red_), blue(blue_), green(green_) {}

    bag operator+(const bag& other) {
        return bag(red + other.red, green + other.green, blue + other.blue);
    }

    bool operator<=(const bag& other) const {
        return red <= other.red && green <= other.green && blue <= other.blue;
    }

    bool operator> (const bag& other) const {
        return red > other.red || green > other.green || blue > other.blue;
    }

    bag max(const bag& other) const {
        return bag(std::max(red, other.red), std::max(green, other.green), std::max(blue, other.blue));
    }

    long power() const {
        return red * green * blue;
    }

};

struct game {
    std::vector<bag> bags;


};

std::vector<game> load_games() {
    std::vector<game> games;
    do_each_input([&](const std::string& line) {
        auto title_tokens = split(line, ":");
        auto game_tokens = split(title_tokens[1], ";");
        game current_game;
        for(const auto& game_token : game_tokens) {
            auto bag_tokens = split(game_token, ",");
            bag current_bag(0, 0, 0);
            for(const auto& bag_token : bag_tokens) {
                auto cubes_tokens = split(bag_token.substr(1), " ");
                if(cubes_tokens[1] == "red") {
                    current_bag.red = std::stoi(cubes_tokens[0]);
                } else if(cubes_tokens[1] == "green") {
                    current_bag.green = std::stoi(cubes_tokens[0]);
                } else if(cubes_tokens[1] == "blue") {
                    current_bag.blue = std::stoi(cubes_tokens[0]);
                }
            }
            current_game.bags.push_back(current_bag);
        }
        games.push_back(current_game);        
    });
    return games;
}

int main() {
    auto games = load_games();
    long counter = 0;
    for(int i = 0; i < games.size(); i++) {
        bag min_bag(0, 0, 0);
        for(const auto& bag : games[i].bags) {
            min_bag = min_bag.max(bag);
        }
        counter += min_bag.power();
    }
    std::cout << "Total: " << counter << std::endl;
    return 0;
}