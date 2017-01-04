#include <iostream>
#include <string>
#include <algorithm>

bool isOneInsertRemove(const std::string& s1, const std::string& s2) {
    auto index1 = 0, index2 = 0;

    while (index2 < s2.size() && index1 < s1.size()) {
        if (s2[index2] != s1[index1]) {
            if (index2 != index1) {
                return false;
            }
            ++index2;
        } else {
            ++index1;
            ++index2;
        }
    }
    return true;
}

bool isOneOperationAway(const std::string& s1, const std::string& s2) {
    int sizeDiff = s1.size() - s2.size();
    if (sizeDiff > 1 || sizeDiff < -1) {
        return false;
    }

    // Replace operation
    if (sizeDiff == 0) {
        bool containDiffChar = false;
        for (auto i = 0; i < s1.size(); ++i) {
            if (s1[i] != s2[i]) {
                if (containDiffChar) {
                    return false;
                }
                containDiffChar = true;
            }
        }
        return true;
    }

    // Insert and remove operations
    if (sizeDiff == 1) { // s2 is the smaller string
        return isOneInsertRemove(s2, s1);
    } else {
        return isOneInsertRemove(s1, s2);
    }
}

int main() {
    std::cout << isOneOperationAway("pale", "ple") << std::endl;
    std::cout << isOneOperationAway("pale", "pales") << std::endl;
    std::cout << isOneOperationAway("pale", "bale") << std::endl;
    std::cout << isOneOperationAway("pale", "bake") << std::endl;
    return 0;
}
