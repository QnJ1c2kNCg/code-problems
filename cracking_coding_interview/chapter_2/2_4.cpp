#include "linked_list.cpp"

// This is O(n), moving elements at the front of a linked list is very cheap
void partition(LinkedList& list, int partition) {
    // This is for a singly linked list
    auto currentNode = list.getHead();
    auto previousNode = list.getHead();

    while (currentNode) {
        if (currentNode->getValue() < partition) {
            // Move to front of the list
            previousNode->setNext(currentNode->getNext());
            currentNode->setNext(list.getHead());

            // Update the head of the list ptr
            list.setHead(currentNode);

            // Update the nodes ptr's
            currentNode = previousNode->getNext();
        } else {
            previousNode = currentNode;
            currentNode = currentNode->getNext();
        }
    }
}

int main() {
    auto list = LinkedList();
    list.add({9});
    list.add({8});
    list.add({7});
    list.add({6});
    list.add({5});
    list.add({4});
    list.add({3});
    list.add({2});
    list.add({1});

    list.print();
    partition(list, 5);
    list.print();

    return 0;
}
