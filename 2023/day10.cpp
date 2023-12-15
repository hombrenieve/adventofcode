#include "common.h"
#include <climits>
#include <queue>
#include <memory>

struct point {
    int x;
    int y;

    point(int y, int x) : x(x), y(y) {}

    bool operator==(const point& other) const {
        return x == other.x && y == other.y;
    }
};

enum class direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    UNKNOWN
};

struct tube {
    char shape;
    point pos;
    int distance;
    bool visited;

    tube(char shape, point pos) : shape(shape), pos(pos), distance(0), visited(false) {}

    bool in_the_loop() {
        return distance > 0;
    }

    bool is_visited() const {
        return visited;
    }

    void visit() {
        visited = true;
    }

    direction get_direction_in_relation_to(const tube& other) {
        if (pos.y == other.pos.y) {
            if (pos.x < other.pos.x) {
                return direction::WEST;
            } else {
                return direction::EAST;
            }
        } else if (pos.x == other.pos.x) {
            if (pos.y > other.pos.y) {
                return direction::SOUTH;
            } else {
                return direction::NORTH;
            }
        } else {
            return direction::UNKNOWN;
        }
    }

    bool is_connected(const tube& other) {
        switch(get_direction_in_relation_to(other)) {
            case direction::NORTH:
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
            case direction::EAST:
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
            case direction::SOUTH:
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
            case direction::WEST:
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
            case direction::UNKNOWN:
                return false;
        }
        return false;
    
    }

};



std::pair<std::vector<std::vector<tube>>, point> load_map() {
    std::vector<std::vector<tube>> tubes;
    point start(0, 0);
    int rows = 0;
    do_each_input([&](const std::string& line) {
        std::vector<tube> row;
        int cols = 0;
        for (char c : line) {
            if(c == 'S') {
                start = point(rows, cols);
            }
            row.emplace_back(c, point(rows, cols));
            cols++;
        }
        tubes.push_back(row);
        rows++;
    });
    return std::pair(tubes, start);
}

std::vector<tube*> expand(std::vector<std::vector<tube>>& tubes, point p) {
    std::vector<tube*> expanded;
    for(point pn: {point(p.y - 1, p.x), point(p.y + 1, p.x), point(p.y, p.x - 1), point(p.y, p.x + 1)}) {
        if(pn.y >= 0 && pn.y < tubes.size() && pn.x >= 0 && pn.x < tubes[pn.y].size()) {
            if(tubes[p.y][p.x].is_connected(tubes[pn.y][pn.x])) {
                expanded.push_back(&tubes[pn.y][pn.x]);
            }
        }
    }
    return expanded;
}


int find_farthest(std::vector<std::vector<tube>>& tubes, const point& start) {
    std::queue<tube*> q;
    tubes[start.y][start.x].distance = 0;
    q.push(&tubes[start.y][start.x]);
    int max_distance = -1;
    while(!q.empty()) {
        tube* p = q.front();
        if(p->distance > max_distance) {
            max_distance = p->distance;
        }
        q.pop();
        //Expand
        auto expanded = expand(tubes, p->pos);
        for(tube* pn : expanded) {
            if(pn->distance == 0) {
                pn->distance = p->distance + 1;
                pn->visit();
                q.push(pn);
            }
        }
    }
    return max_distance;
}

class SqueezeState {
    protected:
        bool in_bounds(std::vector<std::vector<tube>>& tubes, point& p) {
            return p.x >= 0 && p.x < tubes[p.y].size() && p.y >= 0 && p.y < tubes.size();
        }

    public:
        virtual std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) = 0;
        virtual bool is_end() {
            return false;
        }
        virtual tube* result() {
            return nullptr;
        }
};

class SqueezeStateEnd : public SqueezeState {
    public:
        explicit SqueezeStateEnd() = default;
        bool is_end() override {
            return true;
        }

        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override {
            return std::make_unique<SqueezeStateEnd>();
        }
};

class SqueezeStateResult : public SqueezeState {
    tube* result_tube;
    public:
        SqueezeStateResult(tube* result_tube) : result_tube(result_tube) {
            std::cout << "SqueezeStateResult: " << result_tube->shape << std::endl;
        }

        tube* result() override {
            return result_tube;
        }
        bool is_end() override {
            return true;
        }

        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override {
            return std::make_unique<SqueezeStateResult>(result_tube);
        }
};

class SqueezeStateSqueezingNorth : public SqueezeState {
    public:
        explicit SqueezeStateSqueezingNorth() {
            std::cout << "SqueezeStateSqueezingNorth" << std::endl;
        }
        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override;

};
class SqueezeStateSqueezingEast : public SqueezeState {
    public:
        explicit SqueezeStateSqueezingEast() {
            std::cout << "SqueezeStateSqueezingEast" << std::endl;
        }
        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override;
};
class SqueezeStateSqueezingSouth : public SqueezeState {
    public:
    explicit SqueezeStateSqueezingSouth() {
            std::cout << "SqueezeStateSqueezingSouth" << std::endl;
    }
        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override;
};
class SqueezeStateSqueezingWest : public SqueezeState {
    public:
    explicit SqueezeStateSqueezingWest() {
            std::cout << "SqueezeStateSqueezingWest" << std::endl;
    }
        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override;
};
class SqueezeStateUnknown : public SqueezeState {
    public:
        explicit SqueezeStateUnknown() {
            std::cout << "SqueezeStateUnknown" << std::endl;
        }
        std::unique_ptr<SqueezeState> follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) override;
};
            


std::unique_ptr<SqueezeState> SqueezeStateSqueezingNorth::follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) {
    if(!SqueezeState::in_bounds(tubes, p)) {
        return std::make_unique<SqueezeStateEnd>();
    } else if(!tubes[p.y][p.x].in_the_loop()) {
        return std::make_unique<SqueezeStateResult>(&tubes[p.y][p.x]);
    } else {
        switch(tubes[p.y][p.x].shape) {
            case '|':
                p.y--;
                return std::make_unique<SqueezeStateSqueezingNorth>();
            case '7':
                if (p.x + 1 < tubes[p.y].size() && tubes[p.y][p.x + 1].shape == 'F' || p.x + 1 >= tubes[p.y].size()) {
                    p.y--;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::WEST;
                p.x--;
                return std::unique_ptr<SqueezeStateSqueezingWest>(new SqueezeStateSqueezingWest);
            case 'F':
                if (p.x - 1 >= 0 && tubes[p.y][p.x - 1].shape == '7' || p.x - 1 < 0) {
                    p.y--;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::EAST;
                p.x++;
                return std::make_unique<SqueezeStateSqueezingEast>();
            default:
                //case S: Check if is connected and blah blah, not Today
                return std::make_unique<SqueezeStateEnd>();

        }
    }
}

std::unique_ptr<SqueezeState> SqueezeStateSqueezingEast::follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) {
    if(!SqueezeState::in_bounds(tubes, p)) {
        return std::make_unique<SqueezeStateEnd>();
    } else if(!tubes[p.y][p.x].in_the_loop()) {
        return std::make_unique<SqueezeStateResult>(&tubes[p.y][p.x]);
    } else {
        switch(tubes[p.y][p.x].shape) {
            case '-':
                p.x++;
                return std::make_unique<SqueezeStateSqueezingEast>();
            case '7':
                if(p.y - 1 >= 0 && tubes[p.y - 1][p.x].shape == 'J' || p.y - 1 < 0) {
                    p.x++;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::SOUTH;
                p.y++;
                return std::make_unique<SqueezeStateSqueezingSouth>();
            case 'J':
                if(p.y + 1 < tubes.size() && tubes[p.y + 1][p.x].shape == '7' || p.y + 1 >= tubes.size()) {
                    p.x++;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::NORTH;
                p.y--;
                return std::make_unique<SqueezeStateSqueezingNorth>();
            default:
                return std::make_unique<SqueezeStateEnd>();

        }
    }
}


std::unique_ptr<SqueezeState> SqueezeStateSqueezingSouth::follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) {
    if(!SqueezeState::in_bounds(tubes, p)) {
        return std::make_unique<SqueezeStateEnd>();
    } else if(!tubes[p.y][p.x].in_the_loop()) {
        return std::make_unique<SqueezeStateResult>(&tubes[p.y][p.x]);
    } else {
        switch(tubes[p.y][p.x].shape) {
            case '|':
                p.y++;
                return std::make_unique<SqueezeStateSqueezingSouth>();
            case 'J':
                if(p.x + 1 < tubes[p.y].size() && tubes[p.y][p.x + 1].shape == 'L' || p.x + 1 >= tubes[p.y].size()) {
                    p.y++;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::WEST;
                p.x--;
                return std::make_unique<SqueezeStateSqueezingWest>();
            case 'L':
                if(p.x - 1 >= 0 && tubes[p.y][p.x - 1].shape == 'J' || p.x - 1 < 0) {
                    p.y++;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::EAST;
                p.x++;
                return std::make_unique<SqueezeStateSqueezingEast>();
            default:
                return std::make_unique<SqueezeStateEnd>();

        }
    }
}


std::unique_ptr<SqueezeState> SqueezeStateSqueezingWest::follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) {
    if(!SqueezeState::in_bounds(tubes, p)) {
        return std::make_unique<SqueezeStateEnd>();
    } else if(!tubes[p.y][p.x].in_the_loop()) {
        return std::make_unique<SqueezeStateResult>(&tubes[p.y][p.x]);
    } else {
        switch(tubes[p.y][p.x].shape) {
            case '-':
                p.x--;
                return std::make_unique<SqueezeStateSqueezingWest>();
            case 'L':
                if(p.y + 1 < tubes.size() && tubes[p.y + 1][p.x].shape == 'F' || p.y + 1 >= tubes.size()) {
                    p.x--;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::NORTH;
                p.y--;
                return std::make_unique<SqueezeStateSqueezingNorth>();
            case 'F':
                if(p.y - 1 >= 0 && tubes[p.y - 1][p.x].shape == 'L' || p.y - 1 < 0) {
                    p.x--;
                    return std::make_unique<SqueezeStateUnknown>();
                }
                pos = direction::SOUTH;
                p.y++;
                return std::make_unique<SqueezeStateSqueezingSouth>();
            default:
                return std::make_unique<SqueezeStateEnd>();
        }
    }
}


std::unique_ptr<SqueezeState> SqueezeStateUnknown::follow(std::vector<std::vector<tube>>& tubes, point& p, direction& pos) {
    if(!SqueezeState::in_bounds(tubes, p)) {
        return std::make_unique<SqueezeStateEnd>();
    } else if(!tubes[p.y][p.x].in_the_loop()) {
        return std::make_unique<SqueezeStateResult>(&tubes[p.y][p.x]);
    } else { 
        //can we squeeze?
        switch(pos) {
            case direction::NORTH:
                switch(tubes[p.y][p.x].shape) {
                    case 'L':
                        if (p.x - 1 >= 0 && tubes[p.y][p.x - 1].shape == 'J' || p.x - 1 < 0) {
                            p.y--;
                            return std::make_unique<SqueezeStateSqueezingNorth>();
                        }
                    case 'J':
                        if (p.x + 1 < tubes[p.y].size() && tubes[p.y][p.x + 1].shape == 'L' || p.x + 1 >= tubes[p.y].size()) {
                            p.y--;
                            return std::make_unique<SqueezeStateSqueezingNorth>();
                        }
                    default:
                        return std::make_unique<SqueezeStateEnd>();
                }
                break;
            case direction::EAST:
                switch(tubes[p.y][p.x].shape) {
                    case 'F':
                        if (p.y - 1 >= 0 && tubes[p.y - 1][p.x].shape == 'L' || p.y - 1 < 0) {
                            p.x++;
                            return std::make_unique<SqueezeStateSqueezingEast>();
                        }
                    case 'L':
                        if (p.y + 1 < tubes.size() && tubes[p.y + 1][p.x].shape == 'F' || p.y + 1 >= tubes.size()) {
                            p.x++;
                            return std::make_unique<SqueezeStateSqueezingEast>();
                        }
                    default:
                        return std::make_unique<SqueezeStateEnd>();
                }
                break;
            case direction::SOUTH:
                switch(tubes[p.y][p.x].shape) {
                    case 'F':
                        if (p.x - 1 >= 0 && tubes[p.y][p.x - 1].shape == '7' || p.x - 1 < 0) {
                            p.y++;
                            return std::make_unique<SqueezeStateSqueezingSouth>();
                        }
                    case '7':
                        if (p.x + 1 < tubes[p.y].size() && tubes[p.y][p.x + 1].shape == 'F' || p.x + 1 >= tubes[p.y].size()) {
                            p.y++;
                            return std::make_unique<SqueezeStateSqueezingSouth>();
                        }
                    default:
                        return std::make_unique<SqueezeStateEnd>();
                }
                break;
            case direction::WEST:
                switch(tubes[p.y][p.x].shape) {
                    case '7':
                        if (p.y - 1 >= 0 && tubes[p.y - 1][p.x].shape == 'J' || p.y - 1 < 0) {
                            p.x--;
                            return std::make_unique<SqueezeStateSqueezingWest>();
                        }
                    case 'J':
                        if (p.y + 1 < tubes.size() && tubes[p.y + 1][p.x].shape == '7' || p.y + 1 >= tubes.size()) {
                            p.x--;
                            return std::make_unique<SqueezeStateSqueezingWest>();
                        }
                    default:
                        return std::make_unique<SqueezeStateEnd>();
                }
            case direction::UNKNOWN:
                return std::make_unique<SqueezeStateEnd>();
                break;
        
        }
    }
    return std::make_unique<SqueezeStateEnd>();
}

class Squeezer {
    std::vector<std::vector<tube>>& tubes;
    std::unique_ptr<SqueezeState> state;
    point p;
    direction pos;
    

    public:
        Squeezer(std::vector<std::vector<tube>>& tubes, point p, direction pos) : tubes(tubes), p(p), pos(pos), state(std::make_unique<SqueezeStateUnknown>()) {}

        tube* operator()() {
            while(!state->is_end()) {
                state = state->follow(tubes, p, pos);
            }
            return state->result();
        }
};



tube* squeeze(std::vector<std::vector<tube>>& tubes, const point& p, direction pos) {
    tube* pn = nullptr;
    return pn;
}

tube* get_adyacent(std::vector<std::vector<tube>>& tubes, const point& p, direction pos) {
    point pn(p.y, p.x);
    switch(pos) {
        case direction::NORTH:
            pn.y--;
            break;
        case direction::EAST:
            pn.x++;
            break;
        case direction::SOUTH:
            pn.y++;
            break;
        case direction::WEST:
            pn.x--;
            break;
        case direction::UNKNOWN:
            break;
    }
    return Squeezer(tubes, pn, pos)();
}

std::vector<tube*> expand2(std::vector<std::vector<tube>>& tubes, point p) {
    std::vector<tube*> expanded;
    for(direction pos: {direction::NORTH, direction::EAST, direction::SOUTH, direction::WEST}) {
        tube* pn = get_adyacent(tubes, p, pos);
        if(pn != nullptr) {
            expanded.push_back(pn);
        }
    }
    return expanded;

}


int area_enclosed(std::vector<std::vector<tube>>& tubes) {
    std::queue<tube*> q;
    tubes[0][0].visit();
    q.push(&tubes[0][0]);
    while(!q.empty()) {
        tube* p = q.front();
        q.pop();
        //Expand
        auto expanded = expand2(tubes, p->pos);
        for(tube* pn : expanded) {
            if(!pn->is_visited()) {
                pn->visit();
                q.push(pn);
            }
        }
    }
    return std::accumulate(tubes.begin(), tubes.end(), 0, [](int acc, const std::vector<tube>& row) {
        return acc + std::accumulate(row.begin(), row.end(), 0, [](int acc, const tube& p) {
            return acc + (!p.is_visited() ? 1 : 0);
        });
    });
}

int main(int argc, char** argv) {
    auto [tubes, start] = load_map();
    //mark all the tubes that are in the loop
    find_farthest(tubes, start);
    std::cout << area_enclosed(tubes) << std::endl;
    //print found tubes
    for(auto& row : tubes) {
        for(auto& p : row) {
            if(p.visited) {
                if (p.in_the_loop()) {
                    std::cout << "#";
                } else {
                    std::cout << "O";
                }
            } else {
                std::cout << "I";
            }
        }
        std::cout << std::endl;
    }
    return 0;
}