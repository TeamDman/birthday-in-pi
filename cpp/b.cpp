#include <iostream>
#include <fstream>
#include <string>
#include <string.h>
#include <stdio.h>
#include <map>

#include <algorithm>
#include <cstring>

// for mmap:
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>

// https://stackoverflow.com/questions/17925051/fast-textfile-reading-in-c

// why am I doing this, I don't know c[++]


int main() {
    std::ifstream infile("pi-billion.txt");

    std::map<std::string, int> count;
    std::string current;
    char next;




    static const auto BUFFER_SIZE = 16*1024;
    int fd = open("pi-billion.txt", O_RDONLY);
    if (fd == -1)
        return -1;
    
    posix_fadvise(fd, 0, 0, 1); // FDADVICE_SEQUENTIAL

    char buf[BUFFER_SIZE + 1];
    uintmax_t lines = 0;

    while (size_t bytes_read = read(fd, buf, BUFFER_SIZE))
    {
        if (bytes_read == (size_t)-1) exit -1;
        if (!bytes_read) break;
        for (char *p = buf; (p = (char*) memchr(p, "\n", (buf + bytes_read) - p)); ++p)
        {

        }
    }


    // // Pre-fill the first 9 characters
    // int i;
    // for (i=0; i<8; i++) {
    //     infile.get(next);
    //     if (next == '.') { // skip the decimal
    //         i--;
    //         continue;
    //     }
    //     current += next;
    // }

    // count[current] ++;
    // i=0;
    // while (infile.get(next)) {
    //     current = current.substr(1) + next;
    //     count[current]++;
    //     // if (i++ > 100) break;
    // }

    // // Write count for each date to file
    // std::ofstream outfile("./out.txt");
    // for(auto& kv : count) {
    //     outfile << kv.first << "," << kv.second << "\n";
    // }
    // outfile.close();

    // // Exit with code 0
    // return 0;
}