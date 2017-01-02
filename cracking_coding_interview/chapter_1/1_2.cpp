#include <iostream>
#include <string>
#include <algorithm>

bool isPermutation(std::string s1, std::string s2) {
    if (s1.size() != s2.size()) {
        return false;
    }
    std::sort(s1.begin(), s1.end());
    std::sort(s2.begin(), s2.end());

    if (s1 != s2) {
        return false;
    }

    return true;
}

int main() {

    std::cout << "Simon, Bruno\t" << isPermutation("Simon", "Bruno") << std::endl;
    std::cout << "abc, sdfhdsklfhsduabc\t" << isPermutation("abc", "sdfhdsklfhsduabc") << std::endl;
    std::cout << "allo, lloa\t" << isPermutation("allo", "lloa") << std::endl;
    return 0;
}
