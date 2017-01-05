// Quick implementation of a singly linked list

#include <memory>
#include <iostream>

class Node
{
public:
    Node() = delete;
    Node(int value): value_(value) {nextNode_ = nullptr; };
    int getValue() const { return value_; };
    std::shared_ptr<Node> getNext() const { return nextNode_; };
    void setNext(Node node) { nextNode_ = std::make_shared<Node>(node); };
    void setNext(std::shared_ptr<Node> node) { nextNode_ = node; };

private:
    int value_;
    std::shared_ptr<Node> nextNode_;
};

class LinkedList
{
public:
    LinkedList(): size_(0) {
        head_ = nullptr;
        tail_ = nullptr;
    };

    void add(Node node) {
        if (size_ == 0) {
            tail_ = head_ = std::make_shared<Node>(node);
        } else {
            tail_->setNext(node);
            tail_ = tail_->getNext();
        }
        ++size_;
    };

    void print() const {
        std::shared_ptr<Node> currentNode = head_;
        while (currentNode != nullptr) {
            std::cout << currentNode->getValue() << ", ";
            currentNode = currentNode->getNext();
        }
        std::cout << std::endl;
    };

    size_t getSize() const { return size_; };
    std::shared_ptr<Node> getHead() const { return head_; };
    std::shared_ptr<Node> getTail() const { return tail_; };


private:
    size_t size_;
    std::shared_ptr<Node> head_;
    std::shared_ptr<Node> tail_;
};
