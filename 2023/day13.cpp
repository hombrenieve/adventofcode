#include "common.h"

std::vector<std::string> load_map() {
    std::vector<std::string> mirrormap;
    do_each_input([&mirrormap](const std::string& line) {
        mirrormap.push_back(line);
    });
    return mirrormap;
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
        pattern() : themap(load_map()) {}
        
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

int main(int argc, char** argv) {
    pattern p;
    std::cout << p.get_reflection_notes() << std::endl;
    return 0;
}