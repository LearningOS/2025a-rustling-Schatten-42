// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
  () => {
    println!("Check out my macro!");
  };
}

// 宏定义 要在 宏调用 之前
fn main() {
    my_macro!();
}
