#include "common.h"
#include <memory.h>

constexpr int get_card(char id) {
    int value = -1;
    switch(id) {
        case 'A':
            value = 13;
            break;
        case 'K':
            value = 12;
            break;
        case 'Q':
            value = 11;
            break;
        case 'J':
            value = 1;
            break;
        case 'T':
            value = 10;
            break;
        default:
            value = id - '0';
    }
    return value;
}

enum class kind {
    high_card,
    one_pair,
    two_pairs,
    three_of_a_kind,
    full_house,
    four_of_a_kind,
    five_of_a_kind
};

struct hand {
    int cards[14];
    int orig[5];
    int bid;

    hand(const std::string& s, int bid) : bid(bid) {
        memset(cards, 0, sizeof(cards));
        for (int i = 0; i < 5; i++) {
            cards[get_card(s[i])]++;
            orig[i] = get_card(s[i]);
        }
    }

    kind get_kind() const {
        int pairs = 0;
        int three = 0;
        int four = 0;
        int five = 0;
        int jokers = cards[1];
        for (int i = 2; i <= 13; i++) {
            if (cards[i] == 2) {
                pairs++;
            } else if (cards[i] == 3) {
                three++;
            } else if (cards[i] == 4) {
                four++;
            } else if (cards[i] == 5) {
                five++;
            }
        }
        if (five == 1) {
            return kind::five_of_a_kind;
        } else if (four == 1) {
            if(jokers == 1)
                return kind::five_of_a_kind;
            else
                return kind::four_of_a_kind;
        } else if (three == 1 && pairs == 1) {
            return kind::full_house;
        } else if (three == 1) {
            switch(jokers) {
                case 1:
                    return kind::four_of_a_kind;
                case 2:
                    return kind::five_of_a_kind;
                default:
                    return kind::three_of_a_kind;
            }
        } else if (pairs == 2) {
            if (jokers == 1)
                return kind::full_house;
            else
                return kind::two_pairs;
        } else if (pairs == 1) {
            switch(jokers) {
                case 1:
                    return kind::three_of_a_kind;
                case 2:
                    return kind::four_of_a_kind;
                case 3:
                    return kind::five_of_a_kind;
                default:
                    return kind::one_pair;
            }
        } else {
            switch(jokers) {
                case 1:
                    return kind::one_pair;
                case 2:
                    return kind::three_of_a_kind;
                case 3:
                    return kind::four_of_a_kind;
                case 4:
                    return kind::five_of_a_kind;
                case 5:
                    return kind::five_of_a_kind;
                default:
                    return kind::high_card;
            }
        }
    }

    bool operator<(const hand& other) const {
        if (get_kind() != other.get_kind()) {
            return get_kind() < other.get_kind();
        }
        for(int i = 0; i < 5; i++) {
            if (orig[i] != other.orig[i]) {
                return orig[i] < other.orig[i];
            }
        }
        return false;
    }

};

std::vector<hand> load_hands() {
    std::vector<hand> hands;
    do_each_input([&](const std::string& line) {
        auto splitted = split_with_rep(line, ' ');
        hands.push_back(hand(splitted[0], std::stoi(splitted[1])));
    });
    return hands;
}

int main() {
    auto hands = load_hands();
    std::sort(hands.begin(), hands.end());
    long result = 0;
    for(int i = 0; i < hands.size(); i++) {
        result += hands[i].bid * (i+1);
    }
    std::cout << result << std::endl;
    return 0;
}