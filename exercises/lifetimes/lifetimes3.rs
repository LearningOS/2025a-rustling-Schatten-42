// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


// 1. 结构体内使用引用，必须标周期
// 2. 标注的含义：参数一定活得比结构体更久
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
