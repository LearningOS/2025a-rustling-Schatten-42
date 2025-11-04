// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// I AM NOT DONE

// 从外部导入的函数放在这里: 符号为Rust
extern "Rust" { 
    fn my_demo_function(a: u32) -> u32;

    // 上文原话: `可以对这些函数声明应用一些属性来修改链接行为`
    #[link_name = "my_demo_function"] // 告诉链接器:"下面的函数是my_demo_function的别名"
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
  
  // 从本地(Rust项目内部的模块或crates之间)导出函数: 不需要加 extern "Rust" 来导出 (因为默认)
  // 别的语言, 导出时,要加 pub extern "C" fn ...
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]                              // 导出裸符号 my_demo_function, 不要使用符号混淆
    /*pub extern "Rust"*/ fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
