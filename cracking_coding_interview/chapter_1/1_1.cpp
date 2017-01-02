#include <string>
#include <algorithm>
#include <iostream>

bool isAllUniqueChars(std::string s) {
    std::cout << s << '\t';
    std::sort(s.begin(), s.end());
    char previousChar = '_';
    for (auto c : s) {
        if (c == previousChar) {
            std::cout << "is NOT all unique" << std::endl;
            return false;
        }
        previousChar = c;
    }
    std::cout << "is all unique" << std::endl;
    return true;
}

bool isAllUniqueChars1(std::string s) {
    std::cout << s << '\t';

    if (s.size() > 256) {
        return false;
    }

    char map[256]; // This could be size 26 (or a different number depening of the character set)

    // This is O(1) (but require more space)
    for (int i = 0; i < 256; ++i) {
        map[i] = '_';
    }

    // This is O(n)
    for (auto c : s) {
        if (map[c] == c) {
            std::cout << "is NOT all unique" << std::endl;
            return false;
        }
        map[c] = c;
    }
    std::cout << "is all unique" << std::endl;
    return true;
}

int main() {
    isAllUniqueChars("Bruno");
    isAllUniqueChars("Simon");
    isAllUniqueChars("Brruno");
    isAllUniqueChars("SSimon");
    isAllUniqueChars("Brunoo");

    isAllUniqueChars1("Bruno");
    isAllUniqueChars1("Simon");
    isAllUniqueChars1("Brruno");
    isAllUniqueChars1("SSimon");
    isAllUniqueChars1("Brunoo");

    return 0;
}
