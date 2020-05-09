#include <vector>
#include <iostream>
#include <thread>
#include <mutex>

void next_collatz(std::vector<int> * collatz, std::mutex * mtx) {
    std::lock_guard<std::mutex> guard(*mtx);
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
    std::mutex mtx;
    std::vector<int> data = { 12 };
    std::vector<std::thread> poll;

    for(int i = 0; i < 20; ++i) {
        auto t = std::thread(next_collatz, &data, &mtx);
        poll.push_back(std::move(t));
    }

    for(auto& t: poll) {
        t.join();
    }

    for(auto next: data) {
        std::cout << "Next: " << next << '\n';
    }
}
