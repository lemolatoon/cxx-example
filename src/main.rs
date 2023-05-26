#![no_std]
#![feature(lang_items)]
#![feature(start)]

use core::fmt;
use libc_alloc::LibcAlloc;
// sudo ln -s /lib/x86_64-linux-gnu/libstdc++.so.6 /usr/lib/libstdc++.so
#[cxx::bridge]
mod ffi {

    #[namespace = "test_class"]
    unsafe extern "C++" {
        include!("cxx-example/cpp/include/test.hpp");

        type TestClass;

        fn new_test_class(init_val: i64) -> UniquePtr<TestClass>;
        fn getValue(&self) -> i64;
        fn increment(self: Pin<&mut TestClass>);
        fn increment_by(self: Pin<&mut TestClass>, value: i64);
    }

    unsafe extern "C++" {
        fn put(s: &str);
    }
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello, world!");
    let mut test_class = ffi::new_test_class(2);
    assert_eq!(test_class.getValue(), 2);
    test_class.pin_mut().increment();
    assert_eq!(test_class.getValue(), 3);
    test_class.pin_mut().increment_by(2);
    assert_eq!(test_class.getValue(), 5);
    println!("test_class.getValue() = {}", test_class.getValue());
    0
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

pub struct WRITER;

impl WRITER {
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Write for WRITER {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        ffi::put(s);
        Ok(())
    }
}

#[doc(hidden)]
fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER::new().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
