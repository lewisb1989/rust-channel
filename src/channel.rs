use std::sync::atomic::AtomicPtr;
use std::fmt::Debug;

pub struct Producer<T: Debug + Clone> {
    inner: *mut InnerProducer<T>
}

impl<T: Debug + Clone> Producer<T> {
    
    fn new(inner: *mut InnerProducer<T>) -> Self {
        Self {
            inner
        }
    }
    
    pub fn send(&self, item: T) {
        unsafe {
            self.inner.as_mut().unwrap().send(item);
        }
    }
    
    fn get_inner(&self) -> *mut InnerProducer<T> {
        self.inner
    }

}

pub struct InnerProducer<T: Debug> {
    consumers: AtomicPtr<Vec<usize>>,
    queue_ptr: AtomicPtr<Vec<T>>
}

impl<T: Debug + Clone> InnerProducer<T> {

    fn new() -> Self {
        let queue = Box::into_raw(Box::new(Vec::new()));
        let consumers = Box::into_raw(Box::new(Vec::new()));
        Self {
            consumers: AtomicPtr::new(consumers),
            queue_ptr: AtomicPtr::new(queue)
        }
    }
    
    fn get_queue_mut(&mut self) -> &mut Vec<T> {
        unsafe {
            (*self.queue_ptr.get_mut()).as_mut().unwrap()
        }
    }
    
    fn get_queue(&self) -> Vec<T> {
        unsafe {
            (*self.queue_ptr.as_ptr()).as_ref().unwrap().clone()
        }
    }

    fn get_consumers(&self) -> Vec<usize> {
        unsafe {
            (*self.consumers.as_ptr()).as_ref().unwrap().clone()
        }
    }

    fn get_consumers_mut(&mut self) -> &mut Vec<usize> {
        unsafe {
            (*self.consumers.get_mut()).as_mut().unwrap()
        }
    }
    
    fn send(&mut self, item: T) {
        self.get_queue_mut().push(item);
    }
    
    fn register(&mut self) -> usize {
        self.get_consumers_mut().push(0);
        self.get_consumers().len() - 1
    }

    fn update_offset(&mut self, consumer: usize) {
        let consumers = self.get_consumers_mut();
        let offset = consumers.get_mut(consumer).unwrap();
        *offset = *offset + 1;
    }
    
    fn recv(&mut self, consumer: usize) -> Result<Option<T>, String> {
        if consumer < self.get_consumers().len() {
            let queue = self.get_queue();
            let consumers = self.get_consumers();
            let offset = consumers.get(consumer).unwrap();
            if *offset < queue.len() {
            let message = queue.get(*offset).unwrap();
            self.update_offset(consumer);
            Ok(Some(message.clone()))
            } else {
                Ok(None)
            }
        } else {
            Err(String::from("unknown consumer"))
        }
    }

}

pub struct Consumer<T: Debug + Clone> {
    producer: AtomicPtr<InnerProducer<T>>,
    id: usize
}

impl<T: Debug + Clone> Consumer<T> {

    fn new(producer: AtomicPtr<InnerProducer<T>>) -> Self {
        Self {
            producer,
            id: 0
        }
    }
    
    pub fn recv(&self) -> T {
        let mut message = None;
        while message.is_none() {
            match self.get_producer_mut().recv(self.id) {
                Ok(result) => {
                    if let Some(value) = result {
                        message = Some(value);
                    }
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
        message.unwrap()
    }

    fn get_producer_mut(&self) -> &mut InnerProducer<T> {
        unsafe {
            (*self.producer.as_ptr()).as_mut().unwrap()
        }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            Self {
                producer: AtomicPtr::new(*self.producer.as_ptr()),
                id: self.get_producer_mut().register()
            }
        }
    }
    
}

pub fn new<T: Debug + Clone>() -> (Producer<T>, Consumer<T>) {
    let producer = Producer::<T>::new(Box::into_raw(Box::new(InnerProducer::new())));
    let consumer = Consumer::<T>::new(AtomicPtr::new(producer.get_inner()));
    (producer, consumer)
}