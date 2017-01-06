#include <climits>
#include "3_1.cpp"

template <typename T>
class MinStack: public Stack<T> {
public:

    MinStack(): min_(INT_MAX) {}

    T min() const {
        return minStack_.peek();
    }

    void push(T t) {
        Stack<T>::push(t);

        if (t < min_) {
            minStack_.push(t);
            min_ = t;
        }
    }

    // Ignoring the cases where the stack is empty
    T pop() {
        Stack<T>::size_--;

        if (Stack<T>::stack_[Stack<T>::size_] == minStack_.peek()) {
            minStack_.pop();
        }

        return Stack<T>::stack_[Stack<T>::size_];
    }

private:
    Stack<T> minStack_;
    int min_;
};

int main() {
    auto minStack = MinStack<int>();

    minStack.push(4);
    std::cout << minStack.min() << std::endl;
    minStack.push(8);
    std::cout << minStack.min() << std::endl;
    minStack.push(3);
    std::cout << minStack.min() << std::endl;
    minStack.push(1);
    std::cout << minStack.min() << std::endl;
    minStack.push(9);
    std::cout << minStack.min() << std::endl;
    minStack.push(2);
    std::cout << minStack.min() << std::endl;

    minStack.pop();
    std::cout << minStack.min() << std::endl;
    minStack.pop();
    std::cout << minStack.min() << std::endl;
    minStack.pop();
    std::cout << minStack.min() << std::endl;
    minStack.pop();
    std::cout << minStack.min() << std::endl;

    return 0;
}
