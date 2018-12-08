import queue

class Node:
    def __init__(self, n_child, n_meta, children, metadata):
        self.n_child = n_child
        self.n_meta = n_meta
        self.children = children
        self.metadata = metadata

    def __str__(self):
        return 'n_child: {}, n_meta: {}, metadata: {}'.format(self.n_child, self.n_meta, self.metadata)

    def total_metadata(self):
        total = 0
        total += sum(self.metadata)
        for child in self.children:
            total += child.total_metadata()
        return total

    def print_level(self):
        q = queue.Queue()
        q.put(self)
        while not q.empty():
            current = q.get()
            print(current)
            for child in current.children:
                q.put(child)

    def value(self):
        value = 0
        if len(self.children) == 0:
            value += sum(self.metadata)
        else:
            for meta in self.metadata:
                if meta <= len(self.children):
                    value += self.children[meta - 1].value()
        return value

def build_tree_rec(numbers):
    n_child, n_meta = numbers[:2]
    numbers = numbers[2:]
    children = list()
    if n_child == 0:
        metadata = numbers[:n_meta]
    else:
        for i in range(n_child):
            child, numbers = build_tree_rec(numbers)
            children.append(child)
        metadata = numbers[:n_meta]

    return Node(n_child, n_meta, children, metadata), numbers[n_meta:]



with open('input.txt', 'r') as f:
    numbers = list(map(int, f.read().split()))

root, _ = build_tree_rec(numbers)

print('Part 1:', root.total_metadata())
print('Part 2:', root.value())