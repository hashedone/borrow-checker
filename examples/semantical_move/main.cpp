#include <memory>
#include <iostream>

class DestroyMe {
public:
    DestroyMe() {}

    DestroyMe(DestroyMe && orig) {
        std::cout << "Moved\n";
    }

    ~DestroyMe() {
        std::cout << "Destroyed\n";
    }
};

void foo(DestroyMe dm) {
    std::cout << "foo called\n";
}

int main() {
    auto dm = DestroyMe();
    foo(std::move(dm));
    foo(std::move(dm));
}

