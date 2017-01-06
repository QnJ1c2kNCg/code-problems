#include "3_1.cpp"

// Sorted stack using two stacks
class SortedStack {
public:
    SortedStack(): size_(0) {}

    void push(int value) {
        if (dataStack_.getSize() == 0 || value < dataStack_.peek()) {
            dataStack_.push(value);
        } else {
            while (dataStack_.getSize() > 0 && value > dataStack_.peek()) {
                bufferStack_.push(dataStack_.pop());
            }
            bufferStack_.push(value);
            while (bufferStack_.getSize() > 0) {
                dataStack_.push(bufferStack_.pop());
            }
        }
        ++size_;
    }

    int pop() {
        --size_;
        return dataStack_.pop();
    }

    int peek() {
        return dataStack_.peek();
    }

    int getSize() {
        return size_;
    }

private:
    Stack<int> dataStack_;
    Stack<int> bufferStack_;
    int size_;
};

int main() {
    auto sortedStack = SortedStack();

    sortedStack.push(4);
    sortedStack.push(3);
    sortedStack.push(8);
    sortedStack.push(9);
    sortedStack.push(2);
    sortedStack.push(1);
    sortedStack.push(3);
    sortedStack.push(2);
    sortedStack.push(5);
    sortedStack.push(6);

    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;
    std::cout << sortedStack.pop() << std::endl;

    return 0;
}
