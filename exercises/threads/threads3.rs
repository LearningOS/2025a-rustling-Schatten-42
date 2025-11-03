// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}


// 多发送者, 由于线程要拿走tx的所有权 , 必须进行tx.clone() 
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx2 = tx.clone();


    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap(); // tx.send() 返回一个Result
            // 如果所有 tx(含其clone) 都被 drop, 接收循环才会被终止
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel(); // 创建一个通道,此时 mpsc::Sender<T>/ mpsc::Receiver<T> 类型还不确定
    // tx: 发送者, rx: 接收者
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx { // 默认是阻塞式 recv
        println!("Got: {}", received);
        total_received += 1;
        // 当所有tx drop后,循环终止
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
