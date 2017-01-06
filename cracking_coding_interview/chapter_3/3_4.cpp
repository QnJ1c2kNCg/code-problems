// Queue implementation using two stacks
#include "3_1.cpp"

class Queue {
public:
    Queue() {
        normalStackActive_ = true;
    }

    void push(int value) {
        if (!normalStackActive_) {
            swapStacks();
        }
        normalStack_.push(value);
    }

    int pop() {
        if (normalStackActive_) {
            swapStacks();
        }
        return reverseStack_.pop();
    }

    int peek() {
        if (normalStackActive_) {
            swapStacks();
        }
        return reverseStack_.peek();
    }

private:
    // This could be cleaner
    void swapStacks() {
        if (normalStackActive_) {
            while (normalStack_.getSize() > 0) {
                reverseStack_.push(normalStack_.pop());
            }
        } else {
            while (reverseStack_.getSize() > 0) {
                normalStack_.push(reverseStack_.pop());
            }
        }
        normalStackActive_ = !normalStackActive_;
    }

    Stack<int> normalStack_;
    Stack<int> reverseStack_;
    bool normalStackActive_;
};

int main() {
    auto queue = Queue();

    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    queue.push(5);
    queue.push(6);
    queue.push(7);
    queue.push(8);
    queue.push(9);

    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;

    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    queue.push(5);
    queue.push(6);
    queue.push(7);
    queue.push(8);
    queue.push(9);

    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;
    std::cout << queue.pop() << std::endl;

    return 0;
}
