// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
  #[macro_export] // 默认把宏搬到 crate root, 和 pub 不一样
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}


fn main() {
    // self::my_macro!();
    crate::my_macro!();
}
