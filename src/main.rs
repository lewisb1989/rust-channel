mod channel;

use core::time;
use std::thread;

fn main() {
    // create a new channel
    let (producer, consumer) = channel::new::<String>();
    // send a message
    producer.send(String::from("hello"));
    producer.send(String::from("world"));
    // clone the consumer
    let tx1 = consumer.clone();
    // clone it again
    let tx2 = consumer.clone();
    // create wait group
    let mut wait = Vec::new();
    // spaw thread and move tx1
    let handle = thread::spawn(move || {
        // read from the channel
        println!("tx1 (1): {:#?}", tx1.recv());
        thread::sleep(time::Duration::from_millis(100));
        println!("tx1 (2): {:#?}", tx1.recv());
    });
    wait.push(handle);
    // spaw another thread and move tx2
    let handle = thread::spawn(move || {
        // read from the channel
        println!("tx2 (1): {:#?}", tx2.recv());
        thread::sleep(time::Duration::from_millis(100));
        println!("tx2 (2): {:#?}", tx2.recv());
    });
    wait.push(handle);
    // wait for threads to finish
    for item in wait {
        let _ = item.join();
    }
}
