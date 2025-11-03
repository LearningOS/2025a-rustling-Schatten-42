// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);

    // [hint]
    // `for` loops over Option values are more clearly expressed as an `if let`
    // for x in option { [clippy不推荐用循环遍历option](要么0次->None, 要么一次->Some(_))
    //     res += x;
    // }

    if let Some(x) = option { // x 不能替换成 _ , _不会把值绑定出来(是忽略)  
      res += x;
    }

    println!("{}", res);
}
