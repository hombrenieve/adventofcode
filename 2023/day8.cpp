#include "common.h"

struct node {
    std::string name;
    std::string left;
    std::string right;
};

std::map<std::string, node> load_nodes() {
    std::map<std::string, node> nodes;
    do_each_input([&nodes](const std::string &line) {
        node n;
        auto parts = split(line, " = ");
        n.name = parts[0];
        auto links = split(parts[1], ", ");
        n.left = links[0].substr(1);
        n.right = links[1].substr(0, links[1].size() - 1);
        nodes[n.name] = n;
    });
    return nodes;
}


int navigate(std::map<std::string, node> &nodes, const std::string& instructions) {
    node n = nodes["AAA"];
    int steps = 0;
    while(n.name != "ZZZ") {
        auto step = steps % instructions.size();
        if(instructions[step] == 'L') {
            n = nodes[n.left];
        } else {
            n = nodes[n.right];
        }
        steps++;
    }
    return steps;
}


int main() {
    auto instructions = get_line();
    get_line(); // empty line
    auto nodes = load_nodes();
    auto steps = navigate(nodes, instructions);
    std::cout << steps << std::endl;
}