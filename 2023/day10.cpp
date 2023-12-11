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
                q.push(pn);
            }
        }
    }
    return max_distance;
}

std::vector<point> in_the_loop(std::vector<std::vector<pipe>>& pipes, const point& start) {
    find_farthest(pipes, start);
    std::vector<point> in_the_loop;
    std::for_each(pipes.begin(), pipes.end(), [&](std::vector<pipe>& row) {
        std::for_each(row.begin(), row.end(), [&](pipe& p) {
            if(p.distance > 0) {
                in_the_loop.push_back(p.pos);
            }
        });
    });
    return in_the_loop;
}

constexpr bool is_in_loop(const std::vector<point>& the_loop, const point& p) {
    return std::find(the_loop.begin(), the_loop.end(), p) != the_loop.end();
}

int area_enclosed(int width, int height, const std::vector<point>& in_the_loop) {
    enum class status {
        BORDER_IN,
        BORDER_OUT,
        INSIDE,
        OUTSIDE
    };
    int area = 0;
    status s = status::OUTSIDE;
    for(int y = 0; y < height; y++) {
        for(int x = 0; x < width; x++) {
            point p(y, x);
            bool inloop = is_in_loop(in_the_loop, p);
            switch(s) {
                case status::OUTSIDE:
                    if(inloop) {
                        s = status::BORDER_OUT;
                    }
                    break;
                case status::BORDER_IN:
                    if(!inloop) {
                        s = status::OUTSIDE;
                    }
                    break;
                case status::BORDER_OUT:
                    if(!inloop) {
                        s = status::INSIDE;
                    }
                    break;
                case status::INSIDE:
                    if(inloop) {
                        s = status::BORDER_IN;
                    }
                    break;
            }
            if(s == status::INSIDE) {
                area++;
            }
        }
    }
    return area;
}

int main(int argc, char** argv) {
    auto [pipes, start] = load_map();
    auto points_in_the_loop = in_the_loop(pipes, start);
    std::cout << area_enclosed(pipes[0].size(), pipes.size(), points_in_the_loop) << std::endl;
    return 0;
}