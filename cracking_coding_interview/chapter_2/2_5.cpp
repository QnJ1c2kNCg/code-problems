#include "linked_list.cpp"

// If we suppose the the lenght of the lists is n this algorithm is O(n)
LinkedList sumLists(const LinkedList& list1, const LinkedList& list2) {
    auto currentNode = list1.getHead();
    size_t number1 = 0, number2 = 0, iteration = 1;

    while (currentNode) {
        number1 += currentNode->getValue() * iteration;
        iteration *= 10;
        currentNode = currentNode->getNext();
    }

    // Reset variables
    iteration = 1;
    currentNode = list2.getHead();

    while (currentNode) {
        number2 += currentNode->getValue() * iteration;
        iteration *= 10;
        currentNode = currentNode->getNext();
    }

    int sum = number1 + number2;

    auto output = LinkedList();
    while (sum > 0) {
        output.add({sum % 10});
        sum /= 10;
    }

    return output;
}

int main() {
    auto list1 = LinkedList();
    list1.add({7});
    list1.add({1});
    list1.add({6});

    auto list2 = LinkedList();
    list2.add({5});
    list2.add({9});
    list2.add({2});

    auto list3 = sumLists(list1, list2);
    list3.print();

    return 0;
}
