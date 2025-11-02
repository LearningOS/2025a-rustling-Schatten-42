// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


// 1. 闭包会捕获使用到的环境变量: 仅读取=>不可变引用 | 修改=>可变引用 | +move关键字=>对于用到的变量一律夺取所有权
// 2. handle.join().unwrap() 阻塞直至该线程执行完毕
// 3. join() 返回的是 Result<T, Box<dyn Any + Send>>，T 就是闭包返回值, 将 Ok(T) unwrap() 后就是结果 (把线程内的结果安全返回的唯一方法)
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let time = handle.join().unwrap();
        results.push(time);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
