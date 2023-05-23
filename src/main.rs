use std::pin::pin;

use cxx::UniquePtr;
#[cxx::bridge(namespace = "test_class")]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-example/cpp/include/test.hpp");

        type TestClass;

        fn new_test_class(init_val: i64) -> UniquePtr<TestClass>;
        fn getValue(&self) -> i64;
        fn increment(self: Pin<&mut TestClass>);
        fn increment_by(self: Pin<&mut TestClass>, value: i64);
    }
}

fn main() {
    println!("Hello, world!");
    let mut test_class = ffi::new_test_class(2);
    assert_eq!(test_class.getValue(), 2);
    test_class.pin_mut().increment();
    assert_eq!(test_class.getValue(), 3);
    test_class.pin_mut().increment_by(2);
    assert_eq!(test_class.getValue(), 5);
}
