// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// 我就要改签名😤
fn current_favorite_color() -> &'static str {
// fn current_favorite_color() -> &'static mut str {  
  //--> &mut 必须满足拿到的量是独占且可写的
  //--> 字面量 `全局共享`，不满足`独占且可写`

    "blue"
}

/* fn current_favorite_color() -> &'static mut str {
  let mut s = "blue"; // 这样也不行， 这里的 s 是 &str（不可变字符串切片）
                    // 而 &mut str 是另一种类型，不能相互转换
                    // 想`可写`，最方便的是 to_string()
  s

} */