// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
      // 都is_none了 必定panic (clippy能判断这个是会panic的!)
        // my_option.unwrap();
    }

    let my_arr = &[ // [clippy 判断出来缺逗号(comma)]
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5); // resize是就地修改, 返回值嘛,就是一个() -> [clippy 判断出来你可能不知道这个事]
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(5, 0);
    println!("This Vec is empty, see? {:?}", my_empty_vec);


    // error: this looks like you are trying to swap `value_a` and `value_b`
    // clippy看出我想要swap, 但逻辑不对
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
