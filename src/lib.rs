//! mpmc-ringbuf focuses on offering a easy and efficient way to share data between different instance.
//! And we offer the different mode `Fixed`,`Dynamic` to satisfy the demand.
//!
//! see more detail on core module pages
//!
//! simple example
//!
//! single thread
//! ```rust
//!  use mpmc_ringbuf::core::MsgQueue;
//!  let mut msg_queue: MsgQueue<u8> = MsgQueue::new();
//!  let mut writer1 = msg_queue.add_producer();
//!  let mut read1 = msg_queue.add_consumer();
//!  writer1.write(vec![10; 100]);
//!  assert_eq!(read1.size(), 100);
//!  let mut read2 = msg_queue.add_consumer();
//!  assert_eq!(read2.size(), 0);
//!  assert_eq!(msg_queue.get_consumer_count(), 2);
//!  writer1.write(vec![0; 100]);
//!  assert_eq!(read1.size(), 200);
//!  assert_eq!(read2.size(), 100);
//!  read2.read(50);
//!  assert_eq!(read1.size(), 200);
//!  assert_eq!(read2.size(), 50);
//! ```
//!
#![feature(core_panic)]
pub mod core;
