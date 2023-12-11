#include "common.h"
#include <climits>
#include <queue>

struct point {
    int x;
    int y;

    point(int y, int x) : x(x), y(y) {}

    bool operator==(const point& other) const {
        return x == other.x && y == other.y;
    }
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
    bool visited;

    pipe(char shape, point pos) : shape(shape), pos(pos), distance(0), visited(false) {}

    bool in_the_loop() {
        return distance > 0;
    }

    bool is_visited() const {
        return visited;
    }

    void visit() {
        visited = true;
    }

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


int find_farthest(std::vector<std::vector<pipe>>& pipes, const point& start) {
    std::queue<pipe*> q;
    pipes[start.y][start.x].distance = 0;
    q.push(&pipes[start.y][start.x]);
    int max_distance = -1;
    while(!q.empty()) {
        pipe* p = q.front();
        if(p->distance > max_distance) {
            max_distance = p->distance;
        }
        q.pop();
        //Expand
        auto expanded = expand(pipes, p->pos);
        for(pipe* pn : expanded) {
            if(pn->distance == 0) {
                pn->distance = p->distance + 1;
                pn->visit();
                q.push(pn);
            }
        }
    }
    return max_distance;
}

pipe* get_adyacent(std::vector<std::vector<pipe>>& pipes, const point& p, position pos) {
    point pn(p.y, p.x);
    switch(pos) {
        case position::NORTH:
            pn.y--;
            if(pn.y >= 0) {
                if(!pipes[pn.y][pn.x].in_the_loop()) {
                    return &pipes[pn.y][pn.x];
                }
            }
            break;
        case position::EAST:
            pn.x++;
            if(pn.x < pipes[p.y].size()) {
                if(!pipes[pn.y][pn.x].in_the_loop()) {
                    return &pipes[pn.y][pn.x];
                }
            }
            break;
        case position::SOUTH:
            pn.y++;
            if(pn.y < pipes.size()) {
                if (!pipes[pn.y][pn.x].in_the_loop()) {
                    return &pipes[pn.y][pn.x];
                }
            }
            break;
        case position::WEST:
            pn.x--;
            if(p.x >= 0) {
                if(!pipes[pn.y][pn.x].in_the_loop()) {
                    return &pipes[pn.y][pn.x];
                }
            }
            break;
        case position::UNKNOWN:
            break;
    }
    return nullptr;
}

std::vector<pipe*> expand2(std::vector<std::vector<pipe>>& pipes, point p) {
    std::vector<pipe*> expanded;
    for(position pos: {position::NORTH, position::EAST, position::SOUTH, position::WEST}) {
        pipe* pn = get_adyacent(pipes, p, pos);
        if(pn != nullptr) {
            expanded.push_back(pn);
        }
    }
    return expanded;

}


int area_enclosed(std::vector<std::vector<pipe>>& pipes) {
    std::queue<pipe*> q;
    pipes[0][0].visit();
    q.push(&pipes[0][0]);
    while(!q.empty()) {
        pipe* p = q.front();
        q.pop();
        //Expand
        auto expanded = expand2(pipes, p->pos);
        for(pipe* pn : expanded) {
            if(!pn->is_visited()) {
                pn->visit();
                q.push(pn);
            }
        }
    }
    return std::accumulate(pipes.begin(), pipes.end(), 0, [](int acc, const std::vector<pipe>& row) {
        return acc + std::accumulate(row.begin(), row.end(), 0, [](int acc, const pipe& p) {
            return acc + (!p.is_visited() ? 1 : 0);
        });
    });
}

int main(int argc, char** argv) {
    auto [pipes, start] = load_map();
    //mark all the pipes that are in the loop
    find_farthest(pipes, start);
    std::cout << area_enclosed(pipes) << std::endl;
    return 0;
}