// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/*
#[rustfmt::skip]
fn ugly_but_intentional() {badly(); formatted();}
*/


#[rustfmt::skip] // 保持宏手写的格式,不要进行美化格式化
macro_rules! my_macro { // 使用分号隔开多条模式
    () => {
        println!("Check out my macro!");
    };  
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
