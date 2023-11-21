#include <iostream>
#include <optional>
#include <memory>
#include <map>

#include <sstream>
#include <vector>

// for string delimiter
std::vector<std::string> split(std::string s, std::string delimiter) {
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    std::string token;
    std::vector<std::string> res;

    while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos) {
        token = s.substr (pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back (token);
    }

    res.push_back (s.substr (pos_start));
    return res;
}

enum class symbol {
    plus, minus, slash, mult

};

const symbol symbol_from_str(std::string& str) {
    if(str == "+") {
        return symbol::plus;
    }
    if(str == "-") {
        return symbol::minus;
    }
    if(str == "*") {
        return symbol::mult;
    }
    if(str == "/") {
        return symbol::slash;
    }
    std::cerr << "Error unknown symbol: " << str << std::endl;
    exit(1);
}

const int operate(int a, int b, symbol s) {
    switch (s) {
        case symbol::plus:
            return a+b;
        case symbol::minus:
            return a-b;
        case symbol::mult:
            return a*b;
        case symbol::slash:
            return a/b;
        default:
            exit(1);
    }
}

struct monkey;

struct operation {
    std::string a;
    std::string b;
    symbol s;

    operation(std::vector<std::string> op) :
        a(op[1]), b(op[3]), s(symbol_from_str(op[2])) { }

    int execute(std::map<std::string, monkey>& monkeys);
};

struct monkey {
    std::string name;
    std::optional<int> number;
    std::optional<operation> op;

    monkey() {}
    monkey(std::string& name_, operation& op_) : name(name_), op(op_) {}
    monkey(std::string& name_, int n) : name(name_), number(n) {}

    int execute(std::map<std::string, monkey>& monkeys) {
        if(number) {
            return number.value();
        }
        if(!op) {
            std::cerr << "No operation either in " << name << std::endl;
            exit(1);
        }
        this->number = op.value().execute(monkeys);
        return number.value();
    }
};

int operation::execute(std::map<std::string, monkey>& monkeys) {
    int resA = monkeys[a].execute(monkeys);
    int resB = monkeys[b].execute(monkeys);
    return operate(resA, resB, s);
}


void load_monkeys(std::map<std::string, monkey>& monkeys) {
    char buffer[128];
    while(std::cin.getline(buffer, 127)) {
        auto splitted = split(buffer, " ");
        std::string name = splitted[0].substr(0, splitted[0].size()-1);
        if (splitted.size() == 4) {
            operation op(splitted);
            monkeys.emplace(name, monkey{name, op});
        } else {
            monkeys.emplace(name, monkey{name, std::stoi(splitted[1])});
        }
    }
}

int main() {
    std::map<std::string, monkey> monkeys;
    load_monkeys(monkeys);
    int res = monkeys["root"].execute(monkeys);
    std::cout << "Root yells: " << res << std::endl;
    return 0;
}