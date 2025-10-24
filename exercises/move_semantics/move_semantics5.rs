// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100; // 可变的变量
    let y = &mut x; // y 是 x 的可变引用
    *y += 100; // *y 可以修改 x 的值
    let z = &mut x;  // z 也是 x 的可变引用
    *z += 1000; // *z += 1000 相当于 x += 1000
    assert_eq!(x, 1200);
}
