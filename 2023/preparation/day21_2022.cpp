#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <cmath>

#include <sstream>
#include <vector>

#include "../common.h"

enum class symbol {
  plus,
  minus,
  slash,
  mult

};

const symbol symbol_from_str(std::string &str) {
  if (str == "+") {
    return symbol::plus;
  }
  if (str == "-") {
    return symbol::minus;
  }
  if (str == "*") {
    return symbol::mult;
  }
  if (str == "/") {
    return symbol::slash;
  }
  std::cerr << "Error unknown symbol: " << str << std::endl;
  exit(1);
}

const double operate(double a, double b, symbol s) {
  switch (s) {
  case symbol::plus:
    return a + b;
  case symbol::minus:
    return a - b;
  case symbol::mult:
    return a * b;
  case symbol::slash:
    return a / b;
  default:
    exit(1);
  }
}

struct monkey;

struct operation {
  std::string a;
  std::string b;
  symbol s;

  operation(std::vector<std::string> op)
      : a(op[1]), b(op[3]), s(symbol_from_str(op[2])) {}

  double execute(std::map<std::string, monkey> &monkeys);
};

struct monkey {
  std::string name;
  std::optional<double> number;
  std::optional<operation> op;

  monkey() {}
  monkey(std::string &name_, operation &op_) : name(name_), op(op_) {}
  monkey(std::string &name_, double n) : name(name_), number(n) {}

  double execute(std::map<std::string, monkey> &monkeys) {
    if (number) {
      return number.value();
    }
    if (!op) {
      std::cerr << "No operation either in " << name << std::endl;
      exit(1);
    }
    this->number = op.value().execute(monkeys);
    return number.value();
  }
};

double operation::execute(std::map<std::string, monkey> &monkeys) {
  double resA = monkeys[a].execute(monkeys);
  double resB = monkeys[b].execute(monkeys);
  return operate(resA, resB, s);
}

void load_monkeys(std::map<std::string, monkey> &monkeys) {
  auto each_input = [&](const std::string &line) {
    auto splitted = split(line, " ");
    std::string name = splitted[0].substr(0, splitted[0].size() - 1);
    if (splitted.size() == 4) {
      operation op(splitted);
      monkeys.emplace(name, monkey{name, op});
    } else {
      monkeys.emplace(name, monkey{name, std::stof(splitted[1])});
    }
  };
  do_each_input(each_input);
}

void print(std::map<std::string, monkey> &monkeys) {
  for (auto p : monkeys) {
    auto m = p.second;
    std::cout << m.name << ": " << m.number.value() << std::endl;
  }
}

int main() {
  std::map<std::string, monkey> monkeys;
  load_monkeys(monkeys);
  double res = monkeys["root"].execute(monkeys);
  std::cout << "Root yells: " << long(round(res)) << std::endl;
  return 0;
}