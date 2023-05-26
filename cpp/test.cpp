#include "include/test.hpp"
#include "rust/cxx.h"
#include <iostream>
#include <memory>

extern "C"
{
    void __cxa_atexit(){};
    void __stack_chk_fail(){};
    void _Unwind_Resume(){};
}

namespace test_class
{
    TestClass::TestClass(int64_t value) : value_(value) {}

    int64_t TestClass::getValue() const
    {
        return value_;
    }

    void TestClass::increment()
    {
        value_++;
    }

    void TestClass::increment_by(int64_t by)
    {
        value_ += by;
    }

    std::unique_ptr<TestClass> new_test_class(int64_t value)
    {
        return std::make_unique<TestClass>(value);
    }
}

namespace template_test
{
    template <typename T>
    StackVector<T>::StackVector() : size(0) {}

    template <typename T>
    bool StackVector<T>::push(T value)
    {
        if (size == 20)
        {
            return false;
        }
        value_[size] = value;
        size++;
        return true;
    }

    template <typename T>
    T StackVector<T>::pop()
    {
        if (size == 0)
        {
            return nullptr;
        }
        size--;
        return value_[size];
    }

    template <typename T>
    size_t StackVector<T>::len()
    {
        return size;
    }

    std::unique_ptr<StackVector<int32_t>> new_stack_vector()
    {
        return std::make_unique<StackVector<int32_t>>();
    };
}

void put(rust::Str s)
{
    std::cout << std::string(s);
}