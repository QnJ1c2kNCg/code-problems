#include "linked_list.cpp"

int returnMiddleNode(const LinkedList& list) {
    auto runner = list.getHead();
    auto middleNode = list.getHead();
    size_t index = 1;
    while (runner != nullptr) {
        runner = runner->getNext();
        ++index;
        if (index % 2 == 1) {
            middleNode = middleNode->getNext();
        }
    }
    return middleNode->getValue();
}

int main() {
    auto list = LinkedList();
    list.add({1});
    list.add({2});
    list.add({3});
    list.add({4});
    list.add({5});
    list.add({6});
    std::cout << returnMiddleNode(list) << std::endl;
    list.add({7});
    std::cout << returnMiddleNode(list) << std::endl;
    list.add({8});
    std::cout << returnMiddleNode(list) << std::endl;
    list.add({9});
    std::cout << returnMiddleNode(list) << std::endl;
    return 0;
}
