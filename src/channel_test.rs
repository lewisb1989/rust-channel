#[cfg(test)]
mod tests {

    use crate::channel;

    use core::time;
    use std::thread;
    use std::sync::{RwLock, Arc};

    #[test]
    fn test_recv() {
        let (producer, consumer) = channel::new::<u32>();
        let consumer2 = consumer.clone();
        producer.send(1);
        let result = consumer.recv();
        assert_eq!(result, 1 as u32);
        let result = consumer2.recv();
        assert_eq!(result, 1 as u32);
    }

    #[test]
    fn test_recv_blocking() {
        let (producer, consumer) = channel::new::<u32>();
        producer.send(1);
        let messages = Arc::new(RwLock::new(Vec::new()));
        let messages_cloned = messages.clone();
        thread::spawn(move || {
            loop {
                let message = consumer.recv();
                messages_cloned.write().unwrap().push(message);
            }
        });
        thread::sleep(time::Duration::from_secs(2));
        assert_eq!(messages.read().unwrap().len(), 1);
    }

    #[test]
    fn test_try_recv() {
        let (producer, consumer) = channel::new::<u32>();
        producer.send(1);
        let result = consumer.try_recv();
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), 1 as u32);
        let result = consumer.try_recv();
        assert_eq!(result.is_none(), true);
    }

}