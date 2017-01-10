#include <iostream>
#include <climits>
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

void selectionSort(std::vector<int>& v) {
  for (auto i = 0; i < v.size(); ++i) {
    std::pair<int, int> index_min = std::make_pair(-1, INT_MAX);
    for (auto j = i + 1; j < v.size(); ++j) {
      if (v[j] < index_min.second && v[j] < v[i]) {
        index_min.first = j;
        index_min.second = v[j];
      }
    }
    if (index_min.first != -1) {
      swap(v, i, index_min.first);
    }
  }
}

void quickSort(std::vector<int>& v, int left, int right) {
  // In this implementation the pivot is the middle element
  int pivot = v[(left + right) / 2];
  int i = left, j = right;

  while (i <= j) {
    while (v[i] < pivot) { // This while stops when we find a "big" element
      ++i;
    }
    while (v[j] > pivot) { // This while stops when we find a "small" element
      --j;
    }
    // If the if wasn't there we would swap "big" elements from the right for a "small" element from the left
    if (i <= j) {
      swap(v, i, j);
      ++i;
      --j;
    }
  }

  // Those 2 portions didn't get sorted so we use recursion
  if (left < j) {
    quickSort(v, left, j);
  }

  if (i < right) {
    quickSort(v, i, right);
  }
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
  // bubbleSort(v);
  // selectionSort(v);
  quickSort(v, 0, 99);

  for(auto x : v) {
    std::cout << x << std::endl;
  }

  return 0;
}
