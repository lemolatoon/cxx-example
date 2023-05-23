#pragma once
#include <memory>

namespace test_class {
    class TestClass {
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
