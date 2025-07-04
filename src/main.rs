mod channel;
mod channel_test;

use std::thread;
use std::sync::{RwLock, Arc};

fn main() {
    // create a new channel
    let (producer, consumer) = channel::new::<u32>();
    // send messages
    for i in 0..1000 {
        producer.send(i);
    }
    // clone the consumer
    let tx1 = consumer.clone();
    // clone it again
    let tx2 = consumer.clone();
    // create wait group
    let mut wait = Vec::new();
    // count messages
    let tx1_count = Arc::new(RwLock::<u16>::new(0));
    let tx2_count = Arc::new(RwLock::<u16>::new(0));
    let tx1_count_clone = tx1_count.clone();
    let tx2_count_clone = tx2_count.clone();
    // spawn thread and move tx1
    let handle = thread::spawn(move || {
        // read from the channel
        while let Some(message) = tx1.try_recv() {
            *tx1_count_clone.write().unwrap() += 1;
            println!("tx1: {:#?}", message);
        }
    });
    wait.push(handle);
    // spawn another thread and move tx2
    let handle = thread::spawn(move || {
        // read from the channel
        while let Some(message) = tx2.try_recv() {
            *tx2_count_clone.write().unwrap() += 1;
            println!("tx2: {:#?}", message);
        }
    });
    wait.push(handle);
    // wait for threads to finish
    for item in wait {
        let _ = item.join();
    }
    // print message counts
    println!("tx1 message count = {:#?}", tx1_count.read().unwrap());
    println!("tx2 message count = {:#?}", tx2_count.read().unwrap());
    // recv also works
    producer.send(0);
    assert_eq!(consumer.recv(), 0);
}
