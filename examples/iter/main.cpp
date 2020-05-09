#include <vector>
#include <functional>
#include <iostream>

std::function<void ()> printer(const std::vector<int> & data) {
    auto beg = data.begin();
    auto end = data.end();

    return [beg, end]() {
        for(auto it = beg; it != end; ++it) {
            std::cout << "Item: " << *it << '\n';
        }
    };
}

void magic(std::vector<int> & data) {
    int last = data.back();
    while(last != 1) {
        if(last % 2 == 0) {
            data.push_back(last / 2);
        } else {
            data.push_back(last * 3 + 1);
        }
        last = data.back();
    }
}

int main() {
    auto data = std::vector<int> { 871 };
    auto print = printer(data);
    magic(data);
    print();
}
