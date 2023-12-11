#include "common.h"

struct point {
    int x, y;
    point(int x, int y) : x(x), y(y) {}
    point() : x(0), y(0) {}
    bool operator==(const point& p) const {
        return x == p.x && y == p.y;
    }
    int manhattan_distance(const point& p) const {
        return std::abs(x - p.x) + std::abs(y - p.y);
    }
};

std::ostream& operator<<(std::ostream& os, point p) {
    os << "(" << p.x << ", " << p.y << ")";
    return os;
}

std::ostream& operator<<(std::ostream& os, std::vector<point> v) {
    os << "[";
    for (point p : v) {
        os << p << ", ";
    }
    os << "]";
    return os;
}

std::vector<point> load_galaxies() {
    std::vector<point> galaxies;
    int x = 0, y = 0;
    do_each_input([&](std::string line) {
        for (char c : line) {
            if (c == '#') {
                galaxies.push_back(point(x, y));
            }
            x++;
        }
        x = 0;
        y++;
    });
    return galaxies;
}

void expand_y(std::vector<point>& galaxies) {
    //Order by y
    std::sort(galaxies.begin(), galaxies.end(), [](const point& a, const point& b) {
        return a.y < b.y;
    });
    std::vector<point> new_galaxies = galaxies;
    for (int y = 0; y <= galaxies[galaxies.size()-1].y; y++) {
        int j = 0;
        while (j < galaxies.size() && galaxies[j].y < y) {
            j++;
        }
        if (j < galaxies.size()) {
            bool found = false;
            while(j < galaxies.size() && galaxies[j].y == y) {
                found = true;
                j++;
            }
            if (!found) {
                for(int k = 0; k < galaxies.size(); k++) {
                    if(galaxies[k].y > y) {
                        new_galaxies[k].y++;
                    }
                }
            }
        }
    }
    galaxies = new_galaxies;
}

void expand_x(std::vector<point>& galaxies) {
    //Order by y
    std::sort(galaxies.begin(), galaxies.end(), [](const point& a, const point& b) {
        return a.x < b.x;
    });
    std::vector<point> new_galaxies = galaxies;
    for (int x = 0; x <= galaxies[galaxies.size()-1].x; x++) {
        int j = 0;
        while (j < galaxies.size() && galaxies[j].x < x) {
            j++;
        }
        if (j < galaxies.size()) {
            bool found = false;
            while(j < galaxies.size() && galaxies[j].x == x) {
                found = true;
                j++;
            }
            if (!found) {
                for(int k = 0; k < galaxies.size(); k++) {
                    if(galaxies[k].x > x) {
                        new_galaxies[k].x++;
                    }
                }
            }
        }
    }
    galaxies = new_galaxies;
}

std::vector<int> distances(const std::vector<point>& galaxies) {
    std::vector<int> distances;
    for(int i = 0; i < galaxies.size(); i++) {
        for(int j = i+1; j < galaxies.size(); j++) {
            distances.push_back(galaxies[i].manhattan_distance(galaxies[j]));
        }
    }
    return distances;
}

int main() {
    auto galaxies = load_galaxies();
    expand_x(galaxies);
    expand_y(galaxies);
    std::cout << galaxies << std::endl;
    auto distance_pairs = distances(galaxies);
    std::cout << std::accumulate(distance_pairs.begin(), distance_pairs.end(), 0) << std::endl;
    return 0;
}