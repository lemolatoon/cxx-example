#include "include/test.hpp"
#include <memory>

namespace test_class {
TestClass::TestClass(int64_t value): value_(value) {}

int64_t TestClass::getValue() const {
    return value_;
}

void TestClass::increment() {
    value_++;
}

void TestClass::increment_by(int64_t by) {
    value_ += by;
}

std::unique_ptr<TestClass> new_test_class(int64_t value) {
    return std::make_unique<TestClass>(value);
}
}