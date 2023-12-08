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

std::vector<std::string> find_matching_nodes(std::map<std::string, node> &nodes, std::function<bool(node)> func) {
    std::vector<std::string> matching;
    for(auto &p : nodes) {
        if(func(p.second)) {
            matching.push_back(p.first);
        }
    }
    return matching;
}

long navigate(std::map<std::string, node> &nodes, const std::string& instructions) {
    auto simnodes = find_matching_nodes(nodes, [](node n) { return n.name[2] == 'A'; });
    std::vector<int> paths;
    int steps = 0;
    for(int i = 0; i < simnodes.size(); i++) {
        auto n = nodes[simnodes[i]];
        steps = 0;
        while(n.name[2] != 'Z') {
            auto step = steps % instructions.size();
            if(instructions[step] == 'L') {
                n = nodes[n.left];
            } else {
                n = nodes[n.right];
            }
            steps++;
        }
        std::cout << i << " " << steps << std::endl;
        paths.push_back(steps);
    }
    std::cout << "Total: " << instructions.size() << std::endl;
    //The result is the lcm of all paths
    long lcm = paths[0];
    for(int i = 1; i < paths.size(); i++) {
        lcm = std::lcm(lcm, paths[i]);
    }
    return lcm;
}


int main() {
    auto instructions = get_line();
    get_line(); // empty line
    auto nodes = load_nodes();
    auto steps = navigate(nodes, instructions);
    std::cout << steps << std::endl;
}