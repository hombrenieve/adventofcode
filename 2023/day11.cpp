#include "common.h"

struct point {
    long long x, y;
    point(long long x, long long y) : x(x), y(y) {}
    point() : x(0), y(0) {}
    bool operator==(const point& p) const {
        return x == p.x && y == p.y;
    }
    long long manhattan_distance(const point& p) const {
        return std::abs(x - p.x) + std::abs(y - p.y);
    }
};

long long& get_x(point& p) {
    return p.x;
}

long long& get_y(point& p) {
    return p.y;
}

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

void expand(std::vector<point>& galaxies, std::function<long long&(point&)> get) {
    //Order by y
    std::sort(galaxies.begin(), galaxies.end(), [&get](point& a, point& b) {
        return get(a) < get(b);
    });
    std::vector<point> new_galaxies = galaxies;
    for (int y = 0; y <= get(galaxies[galaxies.size()-1]); y++) {
        int j = 0;
        while (j < galaxies.size() && get(galaxies[j]) < y) {
            j++;
        }
        if (j < galaxies.size()) {
            bool found = false;
            while(j < galaxies.size() && get(galaxies[j]) == y) {
                found = true;
                j++;
            }
            if (!found) {
                for(int k = 0; k < galaxies.size(); k++) {
                    if(get(galaxies[k]) > y) {
                        get(new_galaxies[k]) += 999999;
                    }
                }
            }
        }
    }
    galaxies = new_galaxies;
}

std::vector<long long> distances(const std::vector<point>& galaxies) {
    std::vector<long long> distances;
    for(int i = 0; i < galaxies.size(); i++) {
        for(int j = i+1; j < galaxies.size(); j++) {
            distances.push_back(galaxies[i].manhattan_distance(galaxies[j]));
        }
    }
    return distances;
}

int main() {
    auto galaxies = load_galaxies();
    expand(galaxies, get_x);
    expand(galaxies, get_y);
    auto distance_pairs = distances(galaxies);
    std::cout << std::accumulate(distance_pairs.begin(), distance_pairs.end(), 0LL) << std::endl;
    return 0;
}