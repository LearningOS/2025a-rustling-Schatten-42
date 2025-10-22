// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = [0; 1000];
    let a1 = [1u32; 256];
    let a2: [i32; 100] = (0..100).collect::<Vec<_>>().try_into().unwrap(); 

    let a3: [i32; 50] = std::array::from_fn(|i| (i * i) as i32);

    for i in a {
      print!("{} ", i);
    }
    println!("");


    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
