use std::borrow::Borrow;
use std::collections::HashMap;
use std::ptr::write;
use std::sync::{Arc, Mutex};
use std::thread::current;
use crate::core::BufferCacheMode::{dynamic, fixed};

/**
 *@Project: ymir
 *@FileName: core.rs
 *@Author: FakeCore
 *@CreateTime: 2022-12-06 00:42
 *@Description:
 */



pub struct MsgQueue {
    buf:HashMap<String,BufferCache>,
}

pub struct MsgQueueControl<'a>{
    msg_queue:Arc<Mutex<&'a mut MsgQueue>>,
    subscription_name:String,
}

impl MsgQueue{
    pub fn new() -> MsgQueue{
        MsgQueue{
            buf: Default::default()
        }
    }

    pub fn set_subscription(&mut self,channel_name:String){
        if self.buf.contains_key(&channel_name) {
            return
        }
        self.buf.insert(channel_name,BufferCache::new());
        // return MsgQueueControl::new( Arc::new(Mutex::new(self)))
    }

    pub fn get_subscription<'a>(&mut self, channel_name:String) -> Result<MsgQueueControl, String> {
        if !self.buf.contains_key(&channel_name) {
            Err("Key is not exist".to_string())
        } else {
            Ok(MsgQueueControl::new(Arc::new(Mutex::new(self)),channel_name))
        }
    }

    pub fn get_buf<'a>(&mut self, channel_name:String) -> Result<Arc<Mutex<&mut BufferCache>>, String> {
        if self.buf.contains_key(&channel_name) {
            Err("Key is not exist".to_string())
        } else {
            Ok(Arc::new(Mutex::new(self.buf.get_mut(&channel_name).unwrap())))
        }
    }

    pub fn print_hello(&self){
        println!("hello world");
    }
}

impl MsgQueueControl<'_>{
    pub fn new(queue: Arc<Mutex<&mut MsgQueue>>, channel_name:String) ->MsgQueueControl{
        MsgQueueControl{
            msg_queue:queue,
            subscription_name:channel_name,
        }
    }

    pub fn print_hello(&self){
        self.msg_queue.lock().unwrap().print_hello();
    }

    pub fn subscription_name(&self) -> String {
        self.subscription_name.clone()
    }

    pub fn is_exist(&self) -> bool{
        self.msg_queue.lock().unwrap().buf.contains_key(&self.subscription_name)
    }

    pub fn push_data(&mut self,data:Vec<u8>){
    }

    pub fn get_data(&mut self,len:u64)->Vec<u8>{
        vec![]
    }

    pub async fn readable(){

    }


}

#[derive(Clone,Copy,PartialEq,Debug)]
enum BufferCacheMode{
    fixed,
    dynamic,
}

pub struct BufferCache{
    cache:Vec<Vec<u8>>,
    mode:BufferCacheMode,
    buf_length:u64,//cache.size()
    page_size:u64,
    w_index:u64,
    r_index:u64,
    size:u64,
    w_page_index:u64,
    r_page_index:u64,

}

//using capacity()-1 == size() as the sign of buf is full.
impl BufferCache {
    pub fn new () -> BufferCache {
        let page_size = 4096;
        let buf_length = 2;
        let buf_cache = vec![vec![0; page_size]; buf_length];
        BufferCache{
            cache: buf_cache,
            mode: BufferCacheMode::fixed,
            buf_length: buf_length as u64, //default: two buffer blocks
            page_size: page_size as u64, //page size is 4k
            w_index: 0,//
            r_index: 0,
            size: 0,
            w_page_index: 0,
            r_page_index: 0
        }
    }
    //fixed mode if
    pub fn write(&mut self,data:Vec<u8>) -> bool {
        let target_len = data.len() as u64;
        //only fixed mode need to calculate the
        if target_len > self.capacity()-self.size(){
            if self.mode == fixed {
                return false;
            }else if self.mode == dynamic{
                //expand a new vector for store
                self.buf_length += 1;
                self.cache.push(vec![0; self.page_size as usize]);
            }
        }
        let mut index = 0;
        while index != target_len {
            let free_space = self.page_size-self.w_index;
            let mut wrote_size = 0;
            let w_index = self.w_index;
            if free_space > target_len{
                wrote_size = target_len;
                self.w_index += wrote_size;
            }else{
                wrote_size = free_space;
                self.w_page_index = (self.w_page_index+1) % self.buf_length;
                self.w_index = 0;
            }
            index += wrote_size;
            println!("wrote_size:{}",wrote_size);
            for i in 0..wrote_size{
                self.cache[self.w_page_index as usize][(w_index+i) as usize] = data[i as usize];
            }
        }
        self.size += target_len;
        return true;
    }


    // current unconsumed data
    pub fn size(&self) -> u64 {
        return self.size;
    }
    //total buf capacity
    pub fn capacity(&self) -> u64 {
        if self.mode == fixed{
            self.page_size*self.buf_length
        }else{
            //in dynamic mode, capacity is no meaningful
            0
        }
    }

    pub fn read(&mut self, mut length: u64) ->Vec<u8>{
        let mut lens = length;
        //check whether buf has enough data for reading
        let mut res = vec![];
        if self.size() < lens {
            return res
        }
        while lens != 0{
            let mut read_size = 0;
            let read_index_start = self.r_index ;
            let mut read_index_end = self.r_index;
            let cur_page_readable_size = (self.page_size - self.r_index);
            let page_index = self.r_page_index;
            if self.r_page_index == self.w_page_index{
                //in the same page
                if self.r_index > self.w_index {
                    // cache layout
                    //··· free space， --- used space             index
                    // ------------------------------------------  0
                    // ------------------------------------------  1
                    // ------------w_index······r_index----------  2
                    // ------------------------------------------  3
                    // ------------------------------------------  end of cache
                    if cur_page_readable_size > lens {
                        //current page data is enough
                        read_index_end = read_index_start + lens;
                        self.r_index += lens as u64;
                    }else{
                        read_index_end = read_index_start + cur_page_readable_size;
                        self.r_index = 0;
                        self.read_page_add();
                    }
                }else{
                    // cache layout
                    //··· free space， --- used space             index
                    // ··········································  0
                    // ··········································  1
                    // ···········r_index------w_index··········   2
                    // ··········································  3
                    // ··········································  end of cache
                    read_index_end = read_index_start + lens;
                    self.r_index += lens as u64;
                }
            } else {
                // cache layout
                //··· free space， --- used space             index
                // ··········································  0
                // ···r_index--------------------------------  1
                // -------------------------w_index··········  2
                // ··········································  3
                // ··········································  end of cache

                // cache layout
                //··· free space， --- used space             index
                // ------------------------------------------  0
                // ---w_index································  1
                // ·························r_index----------  2
                // ------------------------------------------  3
                // ------------------------------------------  end of cache

                if cur_page_readable_size > lens {
                    read_index_end = read_index_start + lens;
                    self.r_index += lens;
                }else{
                    read_index_end = read_index_start + cur_page_readable_size;
                    self.r_index = 0;
                    self.read_page_add();
                }
            }

            let rs = read_index_start as usize;
            let re = read_index_end as usize;
            res.append(self.cache[page_index as usize][rs..re].to_vec().clone().as_mut());
            lens -= (read_index_end-read_index_start);
        }
        self.size -= length;
        res
    }

    pub fn realAll(&mut self) -> Vec<u8>{
        self.read(self.size())
    }

    pub fn mode(&self) -> BufferCacheMode {
        self.mode
    }

    fn read_page_add(&mut self){
        self.r_page_index = (self.r_page_index +1 )%self.buf_length;
    }

    pub fn setDynamicMode(){

    }
    pub fn setFixedMode(){

    }
}

#[cfg(test)]
mod tests{
    use crate::core::{BufferCache, BufferCacheMode};

    #[test]
    fn test_buff_cache(){
        let mut buf = BufferCache::new();
        assert_eq!(buf.mode(),BufferCacheMode::fixed);
        assert_eq!(buf.size(),0);
        assert_eq!(buf.capacity(),4096 * 2);
        assert_eq!(buf.read(3).len(),0);
        buf.write(vec![10,12]);
        assert_eq!(buf.size(),2);
        assert_eq!(buf.read(2).len(),2);

        buf.write(vec![10,12]);
        buf.write(vec![10,12]);
        buf.write(vec![10,12]);
        buf.write(vec![10,12]);
        buf.write(vec![255,12,1,2,3,4,5,6,2]);

        assert_eq!(buf.size(),17);
    }
}


