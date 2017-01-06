// Simple stack implementation
#include <iostream>
#include <cstddef>

template <typename T>
class Stack {
public:
    Stack() {
        size_ = 0;
        capacity_ = DEFAULT_CAPACITY_;
        stack_ = new T[capacity_];
    }

    ~Stack() {
        delete[] stack_;
    }

    void push(T t) {
        stack_[size_] = t;
        ++size_;
        if (size_ == capacity_) {
            resize();
        }
    }

    // Ignoring the cases where the stack is empty
    T pop() {
        --size_;
        return stack_[size_];
    }

    T peek() const { return stack_[size_ - 1]; }

    std::size_t getSize() const { return size_; }

private:
    void resize() {
        capacity_ *= 2;
        delete[] stack_;
        stack_ = new T[capacity_];
    }

    std::size_t size_;
    std::size_t capacity_;
    const std::size_t DEFAULT_CAPACITY_ = 8;
    T* stack_;
};

int main() {
    auto stack = Stack<int>();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    std::cout << stack.getSize() << std::endl;
    std::cout << stack.pop() << std::endl;
    std::cout << stack.pop() << std::endl;
    std::cout << stack.pop() << std::endl;
    std::cout << stack.pop() << std::endl;
    std::cout << stack.pop() << std::endl;
    std::cout << stack.getSize() << std::endl;

    return 0;
}
