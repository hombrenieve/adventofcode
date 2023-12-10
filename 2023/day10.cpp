#include "common.h"
#include <climits>
#include <queue>

struct point {
    int x;
    int y;

    point(int y, int x) : x(x), y(y) {}
};

enum class position {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    UNKNOWN
};

struct pipe {
    char shape;
    point pos;
    int distance;

    pipe(char shape, point pos) : shape(shape), pos(pos), distance(0) {}

    position get_position_in_relation_to(const pipe& other) {
        if (pos.y == other.pos.y) {
            if (pos.x < other.pos.x) {
                return position::WEST;
            } else {
                return position::EAST;
            }
        } else if (pos.x == other.pos.x) {
            if (pos.y > other.pos.y) {
                return position::SOUTH;
            } else {
                return position::NORTH;
            }
        } else {
            return position::UNKNOWN;
        }
    }

    bool is_connected(const pipe& other) {
        switch(get_position_in_relation_to(other)) {
            case position::NORTH:
                switch(shape) {
                    case '|':
                    case '7':
                    case 'F':
                    case 'S':
                        return other.shape == '|' || other.shape == 'L' || other.shape == 'J';
                    case 'L':
                    case 'J':
                    case '-':
                        return false;
                }
            case position::EAST:
                switch(shape) {
                    case '7':
                    case 'J':
                    case '-':
                    case 'S':
                        return other.shape == '-' || other.shape == 'L' || other.shape == 'F';
                    case 'L':
                    case 'F':
                    case '|':
                        return false;
                }
            case position::SOUTH:
                switch(shape) {
                    case '|':
                    case 'L':
                    case 'J':
                    case 'S':
                        return other.shape == '|' || other.shape == 'F' || other.shape == '7';
                    case 'F':
                    case '7':
                    case '-':
                        return false;                    
                }
            case position::WEST:
                switch(shape) {
                    case '-':
                    case 'L':
                    case 'F':
                    case 'S':
                        return other.shape == '-' || other.shape == 'J' || other.shape == '7';
                    case '7':
                    case 'J':
                    case '|':
                        return false;
                }
            case position::UNKNOWN:
                return false;
        }
        return false;
    
    }

};



std::pair<std::vector<std::vector<pipe>>, point> load_map() {
    std::vector<std::vector<pipe>> pipes;
    point start(0, 0);
    int rows = 0;
    do_each_input([&](const std::string& line) {
        std::vector<pipe> row;
        int cols = 0;
        for (char c : line) {
            
            if(c == 'S') {
                start = point(rows, cols);
            }
            row.emplace_back(c, point(rows, cols));
            cols++;
        }
        pipes.push_back(row);
        rows++;
    });
    return std::pair(pipes, start);
}

std::vector<pipe*> expand(std::vector<std::vector<pipe>>& pipes, point p) {
    std::vector<pipe*> expanded;
    for(point pn: {point(p.y - 1, p.x), point(p.y + 1, p.x), point(p.y, p.x - 1), point(p.y, p.x + 1)}) {
        if(pn.y >= 0 && pn.y < pipes.size() && pn.x >= 0 && pn.x < pipes[pn.y].size()) {
            if(pipes[p.y][p.x].is_connected(pipes[pn.y][pn.x])) {
                expanded.push_back(&pipes[pn.y][pn.x]);
            }
        }
    }
    return expanded;
}


int find_farthest(std::vector<std::vector<pipe>>& pipes, point start) {
    std::queue<pipe*> q;
    pipes[start.x][start.y].distance = 0;
    q.push(&pipes[start.x][start.y]);
    int max_distance = -1;
    while(!q.empty()) {
        pipe* p = q.front();
        if(p->distance > max_distance) {
            max_distance = p->distance;
        } else {
            q.pop();
            continue;
        }
        q.pop();
        //Expand
        auto expanded = expand(pipes, p->pos);
        for(pipe* pn : expanded) {
            if(pn->distance == 0) {
                pn->distance = p->distance + 1;
                std::cout << "Pushing " << pn->shape << " Distance: " << pn->distance << std::endl;
                q.push(pn);
            }
        }
    }
    return max_distance;
}

int main(int argc, char** argv) {
    auto [pipes, start] = load_map();
    std::cout << find_farthest(pipes, start) << std::endl;
    return 0;
}