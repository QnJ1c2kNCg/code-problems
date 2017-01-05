#include <unordered_set>
#include "linked_list.cpp"

// This is O(n) but require extra memory for the set
void removeDups(LinkedList& list) {
    auto set = std::unordered_set<int>();
    auto currentNode = list.getHead();
    auto previousNode = list.getHead();
    while (currentNode != nullptr) {
        if (set.count(currentNode->getValue())) {
            // remove current node (ignoring the cases of head and tail)
            previousNode->setNext(currentNode->getNext());
        } else {
            // add the value to the set
            set.insert(currentNode->getValue());
            previousNode = currentNode;
        }
        currentNode = currentNode->getNext();
    }
}

// This is O(n^2) but you do not need an extra data structure
void removeDups1(LinkedList& list) {
    auto currentNode = list.getHead();
    auto previousNode = list.getHead();
    bool removed = false;
    while (currentNode != nullptr) {
        auto runner = currentNode->getNext();
        while (runner != nullptr) {
            if (runner->getValue() == currentNode->getValue()) {
                removed = true;
                break;
            } else {
                // move the runner
                runner = runner->getNext();
            }
        }
        if (removed) {
            previousNode->setNext(currentNode->getNext());
        } else {
            previousNode = currentNode;
        }
        removed = false;
        currentNode = currentNode->getNext();
    }
}

int main() {
    auto list = LinkedList();
    list.add({1});
    list.add({2});
    list.add({3});
    list.add({4});
    list.add({4});
    list.add({2});
    list.add({2});
    list.add({4});
    list.add({5});

    list.print();
    removeDups(list);
    list.print();

    return 0;
}
