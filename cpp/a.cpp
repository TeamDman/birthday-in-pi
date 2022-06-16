#include <iostream>
#include <fstream>
#include <string>
#include <string.h>
#include <stdio.h>
#include <map>

int main() {
    std::ifstream infile("pi-billion.txt");

    std::map<std::string, int> count;
    std::string current;
    char next;

    // Pre-fill the first 9 characters
    int i;
    for (i=0; i<8; i++) {
        infile.get(next);
        if (next == '.') { // skip the decimal
            i--;
            continue;
        }
        current += next;
    }

    count[current] ++;
    i=0;
    while (infile.get(next)) {
        current = current.substr(1) + next;
        count[current]++;
        // if (i++ > 100) break;
    }

    // Write count for each date to file
    std::ofstream outfile("./out.txt");
    for(auto& kv : count) {
        outfile << kv.first << "," << kv.second << "\n";
    }
    outfile.close();

    // Exit with code 0
    return 0;
}