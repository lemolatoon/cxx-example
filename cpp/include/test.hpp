#pragma once
#include <memory>
#include "rust/cxx.h"

namespace test_class
{
    class TestClass
    {
    private:
        int64_t value_;

    public:
        TestClass(int64_t);
        int64_t getValue() const;
        void increment();
        void increment_by(int64_t);
    };

    std::unique_ptr<TestClass> new_test_class(int64_t);
}

namespace template_test
{
    template <typename T>
    class StackVector
    {
    private:
        T value_[20];
        size_t size;

    public:
        StackVector();
        bool push(T);
        T pop();
        size_t len();
    };

    template <typename T>
    std::unique_ptr<StackVector<T>> new_stack_vector();
}

void put(rust::Str s);