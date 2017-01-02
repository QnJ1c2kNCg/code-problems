#include <iostream>
#include <string>

bool isPermutationPalindrome(const std::string& s) {
    // We assume that the string contains only lower case letters
    // This is O(1)
    int map[26];
    for (auto i = 0; i < 26; ++i) {
        map[i] = 0;
    }

    // This is O(n)
    for (auto c : s) {
        map[c - 'a'] = map[c - 'a'] + 1;
    }

    bool containOdd = false;

    // This is worst case O(26) = O(1)
    for (auto i = 0; i < 26; ++i) {
        if (map[i] % 2 != 0) {
            if (containOdd) {
                return false;
            }
            containOdd = true;
        }
    }

    return true;
}

int main() {
    std::cout << "tactcoa\t" << isPermutationPalindrome("tactcoa") << std::endl;
    std::cout << "a\t" << isPermutationPalindrome("a") << std::endl;
    std::cout << "abcd\t" << isPermutationPalindrome("abcd") << std::endl;
    std::cout << "laval\t" << isPermutationPalindrome("laval") << std::endl;
    std::cout << "llaav\t" << isPermutationPalindrome("llaav") << std::endl;

    return 0;
}
