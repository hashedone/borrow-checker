#include <vector>
#include <iostream>
#include <thread>

void next_collatz(std::vector<int> * collatz) {
    auto prev = collatz->back();
    if(prev != 1) {
        if(prev % 2 == 0) {
            collatz->push_back(prev / 2);
        } else {
            collatz->push_back(prev * 3 + 1);
        }
    }
}

int main() {
    std::vector<int> data = { 12 };
    std::vector<std::thread> poll;

    for(int i = 0; i < 20; ++i) {
        auto t = std::thread(next_collatz, &data);
        poll.push_back(std::move(t));
    }

    for(auto& t: poll) {
        t.join();
    }

    for(auto next: data) {
        std::cout << "Next: " << next << '\n';
    }
}
