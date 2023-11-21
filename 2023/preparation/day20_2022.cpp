#include <iostream>
#include <vector>
#include <memory>

struct node {
    const int data;
    std::shared_ptr<node> prev;
    std::shared_ptr<node> next;

    node(const int d) : data(d) { }

    template<typename num>
    static const int get_adv(num n, size_t max) {
        int adv = std::abs(n) % max;
        return (n < 0)? -adv : adv;
    }

    std::shared_ptr<node> advance(int n) {
        if(n == 0) {
            return this->prev->next;
        }
        if(n < 0) {
            return this->prev->advance(n+1);
        }
        return this->next->advance(n-1);
    }

    void remove() {
        prev->next = this->next;
        next->prev = this->prev;
    }

    void insert_next(std::shared_ptr<node> cur) {
        cur->next = this->next;
        cur->prev = this->prev->next;
        this->next->prev = cur;
        this->next = cur;
    }

    void insert_prev(std::shared_ptr<node> cur) {
        cur->prev = this->prev;
        cur->next = this->prev->next;
        this->prev->next = cur;
        this->prev = cur;
    }
};

void load_vector(std::vector<std::shared_ptr<node>>& nodes) {
    char buffer[32];
    while(std::cin.getline(buffer, 32)) {
        int d = std::stoi(std::string(buffer));
        nodes.push_back(std::make_shared<node>(d));
    }
    for(int i = 1; i < nodes.size()-1; i++) {
        nodes[i]->prev = nodes[i-1];
        nodes[i]->next = nodes[i+1];
    }
    nodes[0]->prev = nodes[nodes.size()-1];
    nodes[0]->next = nodes[1];
    nodes[nodes.size()-1]->prev = nodes[nodes.size()-2];
    nodes[nodes.size()-1]->next = nodes[0];
}

void print(std::shared_ptr<node> cur) {
    std::cout << cur->data << ", ";
    auto next = cur->next;
    while(next != cur) {
        std::cout << next->data << ", ";
        next = next->next;
    }
    std::cout << std::endl;
}

void mix(std::vector<std::shared_ptr<node>>& nodes) {
    for(int i = 0; i < nodes.size(); i++) {
        auto cur = nodes[i];
        if(cur->data == 0) continue;
        cur->remove();
        auto pos = cur->advance(node::get_adv(cur->data, nodes.size()-1));
        if(cur->data < 0) {
            pos->insert_prev(cur);
        } else {
            pos->insert_next(cur);
        }
    }
}

int calculate_result(std::shared_ptr<node> node, size_t max) {
    auto cur = node;
    while(cur->data != 0) { cur = cur->next; }
    return cur->advance(node::get_adv(1000, max))->data+
        cur->advance(node::get_adv(2000, max))->data+
        cur->advance(node::get_adv(3000, max))->data;
}

int main() {
    std::vector<std::shared_ptr<node>> nodes;
    load_vector(nodes);
    mix(nodes);
    std::cout << "Result is " << calculate_result(nodes[0], nodes.size()) << std::endl; 
    std::cout << "List size: " << nodes.size() << std::endl;
    return 0;
}