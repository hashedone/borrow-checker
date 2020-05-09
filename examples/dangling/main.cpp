#include <iostream>

int & dangling(int arg) {
    return arg;
}

int main() {
    int answer = 42;
    auto ref = dangling(answer);
    std::cout << "Answer: " << ref << '\n';
}

