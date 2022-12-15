mod core;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::process::exit;
    use std::rc::Rc;
    use std::thread;
    use crate::core::{MsgQueue, MsgQueueReader};
    use super::*;
    #[test]
    fn it_works() {
        // let mut msg_queue = MsgQueue::new();
        // msg_queue.set_subscription("hi".to_string());
        // let control = match msg_queue.get_subscription("hi".to_string()){
        //     Ok(control) =>{ control },
        //     Err(str) => {panic!("err:{}",str)}
        // };
        // control.print_hello();
        // println!("subscription-name:{}",control.subscription_name());
        // println!("exist:{}",control.is_exist());
    }

    #[test]
    fn test_msg_queue() {
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
    }

    #[test]
    fn test_multi_thread_msg_queue() {
        // let mut msg_queue:Rc<RefCell<MsgQueue<u8>>> = Rc::new(RefCell::new(MsgQueue::new()));
        // let m1 = msg_queue.clone();
        // let m2 = msg_queue.clone();
        // let mut c1 = (*msg_queue).get_mut().add_consumer();
        // let mut c2 = (*msg_queue).get_mut().add_consumer();
        // let t1 = thread::spawn(move || {
        //     let p = (*m1).borrow().add_producer();
        //     for i in 0..100{
        //         p.write(vec![0;5]);
        //     }
        // });
        //
        // let t2 = thread::spawn(move || {
        //     let p = (*m1).borrow().add_producer();
        //     for i in 0..100{
        //         p.write(vec![0;5]);
        //     }
        // });
        // t1.join();
        // t2.join();
        // assert_eq!(c1.size(),1000);
        // assert_eq!(c2.size(),1000);
    }

}
