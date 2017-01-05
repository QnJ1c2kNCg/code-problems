#include "linked_list.cpp"

// O(n) time and O(1) space (k = 0 is the last element)
int findKthToLastElement(const LinkedList& list, size_t k) {
    // We assume that k is always smaller than the size of the list

    size_t distance = 0;
    auto currentNode = list.getHead();
    auto kthNode = list.getHead();

    while (currentNode != nullptr) {
        if (distance > k) {
            kthNode = kthNode->getNext();
            --distance;
        }
        currentNode = currentNode->getNext();
        ++distance;
    }

    return kthNode->getValue();
}

int main() {
    auto list = LinkedList();

    list.add({1});
    list.add({2});
    list.add({3});
    list.add({4});
    list.add({5});
    list.add({6});
    list.add({7});
    list.add({8});
    list.add({9});

    std::cout << findKthToLastElement(list, 0) << std::endl;
    std::cout << findKthToLastElement(list, 1) << std::endl;
    std::cout << findKthToLastElement(list, 2) << std::endl;
    std::cout << findKthToLastElement(list, 3) << std::endl;
    std::cout << findKthToLastElement(list, 9) << std::endl;

    return 0;
}
