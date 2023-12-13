#include "common.h"

std::vector<std::vector<std::string>> load_maps() {
    std::vector<std::vector<std::string>> mirrormaps;
    std::vector<std::string> mirrormap;
    while(do_each_input_until_empty([&mirrormap](const std::string& line) {
        mirrormap.push_back(line);
    })) {
        mirrormaps.push_back(mirrormap);
        mirrormap.clear();
    }
    return mirrormaps;
}


class pattern {
    std::vector<std::string> themap;

    bool is_reflected_row(int index) {
            int i = index, j = index + 1;
            int reflected = 0;
            while (j < themap.size() && i >= 0) {
                if(themap[i] != themap[j]) return false;
                reflected++;     
                j++;
                i--;           
            }
            return reflected > 0;
        }

        bool is_reflected_col(int index) {
            int i = index, j = index + 1;
            int reflected = 0;
            while (j < themap[0].size() && i >= 0) {
                for(int k = 0; k < themap.size(); k++) {
                    if(themap[k][i] != themap[k][j]) return false;
                }
                reflected++;
                j++;
                i--;           
            }
            return reflected > 0;
        }

    public:
        pattern(const std::vector<std::string> &amap) : themap(amap) {}
        
        int get_reflection_notes() {
            int notes = 0;
            for(int i = 0; i < themap.size(); i++) {
                if(is_reflected_row(i)) {
                    notes += (i+1) * 100;
                }
            }
            for(int j = 0; j < themap[0].size(); j++) {
                if(is_reflected_col(j)) {
                    notes += j+1;
                }
            }
            return notes;
        }
};

class part1 {
    std::vector<pattern> patterns;
    public:
        part1() {
            std::vector<std::vector<std::string>> mirrormaps(load_maps());
            for(auto &map : mirrormaps) {
                patterns.push_back(pattern(map));
            }    
        }

        int get_total_reflection_notes() {
            int total = 0;
            for(auto &p : patterns) {
                total += p.get_reflection_notes();
            }
            return total;
        }
};

int main(int argc, char** argv) {
    part1 p;
    std::cout << p.get_total_reflection_notes() << std::endl;
    return 0;
}