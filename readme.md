# mpsc-ringbuf
A Rust MPMC(multiple producer and multiple consumers) ringbuf queue



## Usage

single thread

```rust
 				let mut msg_queue:MsgQueue<u8> = MsgQueue::new();
        let mut writer1 = msg_queue.add_producer();
        let mut read1 = msg_queue.add_consumer();
        writer1.write(vec![10;100]);
        println!("{}",msg_queue.get_consumer_count());
        println!("{}",read1.size());
        assert_eq!(read1.size(),100);
        let mut read2 = msg_queue.add_consumer();
        assert_eq!(read2.size(),0);
        assert_eq!(msg_queue.get_consumer_count(),2);
        writer1.write(vec![0;100]);
        assert_eq!(read1.size(),200);
        assert_eq!(read2.size(),100);
        read2.read(50);
        assert_eq!(read1.size(),200);
        assert_eq!(read2.size(),50);
```

multi-thread

```rust
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
        assert_eq!(msg_queue.lock().unwrap().get_consumer_count(),2);
        let t1 = thread::spawn(move || {
            let mut msg_lock = (*m1).lock().unwrap();
            println!("get lock1");
            let p = msg_lock.add_producer();
            for i in 0..100{
                p.write(vec![0;5]);
            }
        });

        let t2 = thread::spawn(move || {
            let mut msg_lock = (*m2).lock().unwrap();
            println!("get lock1");
            let p = msg_lock.add_producer();
            for i in 0..100{
                p.write(vec![0;5]);
            }
        });
        t1.join();
        t2.join();
        {
            let mut msg_lock = (*msg_queue).lock().unwrap();
            assert_eq!(msg_lock.get_consumer_count(),2);
            let mut c1 = msg_lock.get_consumer(c1_id);
            let mut c2 = msg_lock.get_consumer(c2_id);
            println!("size: {} {}",c1.size(),c2.size());
            assert_eq!(c1.size(),1000);
            assert_eq!(c2.size(),1000);
        }
```



## feature

