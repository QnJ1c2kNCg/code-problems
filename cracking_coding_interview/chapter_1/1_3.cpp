#include <string>
#include <iostream>

// This is O(n)
void urlIFY(std::string& s) {
    auto index = s.find_last_not_of(" ");
    auto i = s.size() - 1;
    while (i > 0) {
        if (s[index] != ' ') {
            s[i] = s[index];
        } else {
            s[i--] = '0';
            s[i--] = '2';
            s[i] = '%';
        }
        --i;
        --index;
    }
}

int main() {
    // We assume that the string is already the updated size
    std::string s = "Mr John Smith    ";
    urlIFY(s);
    std::cout << s << std::endl;
    return 0;
}
