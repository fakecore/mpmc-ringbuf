mod core;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{MsgQueue, MsgQueueReader};
    use std::cell::RefCell;
    use std::process::exit;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_single_thread_msg_queue() {
        let mut msg_queue: MsgQueue<u8> = MsgQueue::new();
        let mut writer1 = msg_queue.add_producer();
        let mut read1 = msg_queue.add_consumer();
        writer1.write(vec![10; 100]);
        println!("{}", msg_queue.get_consumer_count());
        println!("{}", read1.size());
        assert_eq!(read1.size(), 100);
        let mut read2 = msg_queue.add_consumer();
        assert_eq!(read2.size(), 0);
        assert_eq!(msg_queue.get_consumer_count(), 2);
        writer1.write(vec![0; 100]);
        assert_eq!(read1.size(), 200);
        assert_eq!(read2.size(), 100);
        read2.read(50);
        assert_eq!(read1.size(), 200);
        assert_eq!(read2.size(), 50);
    }

    #[test]
    fn test_multi_thread_msg_queue() {
        let mut msg_queue: Arc<Mutex<MsgQueue<u8>>> = Arc::new(Mutex::new(MsgQueue::new()));
        let m1 = msg_queue.clone();
        let m2 = msg_queue.clone();
        let mut c1_id = 0;
        let mut c2_id = 0;
        {
            let mut msg_lock = (*msg_queue).lock().unwrap();
            let mut c1 = msg_lock.add_consumer();
            let mut c2 = msg_lock.add_consumer();
            c1_id = c1.id();
            c2_id = c2.id();
        }
        assert_eq!(msg_queue.lock().unwrap().get_consumer_count(), 2);
        let t1 = thread::spawn(move || {
            let mut msg_lock = (*m1).lock().unwrap();
            println!("get lock1");
            let p = msg_lock.add_producer();
            for i in 0..100 {
                p.write(vec![0; 5]);
            }
        });

        let t2 = thread::spawn(move || {
            let mut msg_lock = (*m2).lock().unwrap();
            println!("get lock1");
            let p = msg_lock.add_producer();
            for i in 0..100 {
                p.write(vec![0; 5]);
            }
        });
        t1.join();
        t2.join();
        {
            let mut msg_lock = (*msg_queue).lock().unwrap();
            assert_eq!(msg_lock.get_consumer_count(), 2);
            let mut c1 = msg_lock.get_consumer(c1_id);
            let mut c2 = msg_lock.get_consumer(c2_id);
            println!("size: {} {}", c1.size(), c2.size());
            assert_eq!(c1.size(), 1000);
            assert_eq!(c2.size(), 1000);
        }
    }

    #[test]
    fn test_string() {
        let mut msg_queue = Rc::new(RefCell::new(MsgQueue::<String>::new()));
        let mut c1 = msg_queue.borrow_mut().add_consumer();
        let mut p1 = msg_queue.borrow_mut().add_producer();
        p1.write(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(c1.size(), 2);
        let data = c1.read_all();
        assert_eq!(c1.size(), 0);
        assert_eq!(data.len(), 2);
        assert_eq!(data.get(0).unwrap().to_string(), "hello".to_string());
        assert_eq!(data.get(1).unwrap().to_string(), "world".to_string());
        for i in data {
            print!("{:?} ", i);
        }
    }
}
