#include <iostream>
#include <vector>

void swap(std::vector<int>& v, size_t index1, size_t index2) {
  int temp = v[index1];
  v[index1] = v[index2];
  v[index2] = temp;
}

void bubbleSort(std::vector<int>& v) {
  bool swapped = false;
  do {
    swapped = false;
    for (auto j = 0; j < v.size() - 1; ++j) {
      if (v[j] > v[j + 1]) {
        swap(v, j, j + 1);
        swapped = true;
      }
    }
  } while (swapped);
}

int main() {

  auto v = std::vector<int>();
  v.reserve(100);
  for (auto i = 100; i > 0; --i) {
    v.push_back(i);
  }

  for(auto x : v) {
    std::cout << x << std::endl;
  }

  std::cout << "Sorting" << std::endl;
  bubbleSort(v);

  for(auto x : v) {
    std::cout << x << std::endl;
  }

  return 0;
}
