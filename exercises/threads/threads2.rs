// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

// Mutex是一个`盒子`(智能指针), 数据装在里面就能用于多线程 mut 了 => 还需要 Arc 传给 N 条线程
//  (单独 Arc 只能实现多线程 & , 而非 &mut )

/*
Mutex 本身不是 DerefMut 那种“指针”，但它提供的 .lock() 返回的 MutexGuard<T> 实现了 Deref/DerefMut，
用起来像一个“带生命周期、自动解锁的智能指针”

let counter = Mutex::new(0); // 盒子诞生，里面放着 i32 
{   // 打开盒子，拿到“唯一钥匙” (上锁):
    let mut num = counter.lock().unwrap(); // lock().unwrap() 返回智能指针 MutexGuard<T>,实现了Deref 和 Drop,可以直接拿来访问(需要*解引用后修改数据)
    *num += 1;                             // 改的是盒子里的数据
}                                          // 钥匙自动归还(实现了drop,离开作用域自动drop)，锁被 drop (自动 unlock)

*/

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            {
            let mut guard = status_shared.lock().unwrap();
            // TODO: You must take an action before you update a shared value
            (*guard).jobs_completed += 1; // 也可以不加*,编译器找不到 guard.xxx 的时候会自动加 *
            } // 显式drop MutexGuard, 这样就能move整个Arc(包着Mutex;了)
            
            // status_shared
            // 不需要返回值: 我们可以直接访问外部的主 Arc 
            // 此处如果不先drop 也不能返回 status_shared:  move必须发生在借用结束之后, 此时的guard 正在借用status_shared
            // 即MutexGuard 活着的时候,Mutex不能被移动

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed); // 想读到里面的数据,必须再锁一次(即便此时已经在单线程内)
    }
}
