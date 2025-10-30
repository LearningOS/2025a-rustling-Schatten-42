// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// 1. 编译器在担心什么？
// rust 并不知道该函数将要返回的引用是 x 还是 y
// 因为编译器需要知道这些，来确保函数调用后的引用生命周期分析:
// - 如果返回x，那么编译器的`借用检查`就会需要检查：
// “ 返回值在后面有没有被使用？ ”
// “ x 的声明周期是否延续到了 返回值被使用的时候？ ”
// 如果否，就会导致悬垂引用
// 但实际上编译器并不知道究竟返回的是 x 还是 y
// 也就不明确究竟要分析 x、y中哪一个的声明周期

// 2. 标注声明周期解决了什么问题？
// 给编译器一个承诺：
// “放心，我写的代码会让:
//    返回值会活在x y的交集，它被使用的时候x y一定活着 ”
//   `end-ret <= min{ end-x, end-y}`
// => 这样，编译器就知道“嗯，你的承诺合法，接下来让我看看调用者是不是这么写的”
// ==> 随后，编译器拿着这个承诺检查函数调用部分的代码：
// “如果代码实际的写法并不符合你的标注，我就报错！”

/* 需要注意的点如下：
- 和泛型一样，使用生命周期参数，需要先声明 <'a>，此时'a的生命并未确定
- 实际调用的时候，参数 x，y 传入时，编译器就拿到了'a 的真正长度：
    “参数传入了！'a 是它们生命的交集，让我算算这个'a ”
    “接下来我要检查一下编写函数之人的承诺：后续返回值的生命一定不能超过这个'a ”

- 所以写完函数并不是万事大吉/
  你还需要确保调用时，代码真的如你所承诺的那样，不然编译器就会报错
*/

// “longest：凡是调用我的人,都必须遵守`返回值不如参数交集活得久`的规则”
fn longest<'k>(x: &'k str, y: &'k str) -> &'k str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 3. 什么时候需要标注？

// “只要函数返回（或输出）了借用型数据（&T、&mut T、包含 & 的结构体、trait 对象 …）
// 而编译器又无法从签名本身唯一地推断出这些借用的来源，就必须标注”
// （eg: 传入&x，&y，就算 函数确保返回的就是 x 能体现返回值的唯一,也必须标注)
//  原因：编译器只看签名,签名里有两个引用,返回的是传入的引用,就让你标

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
