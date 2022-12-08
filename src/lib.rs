mod core;

left + right
}

#[cfg(test)]
mod tests {
    use std::process::exit;
    use crate::core::{MsgQueue};
    use super::*;
    #[test]
    fn it_works() {
        let mut msg_queue = MsgQueue::new();
        msg_queue.set_subscription("hi".to_string());
        let control = match msg_queue.get_subscription("hi".to_string()){
            Ok(control) =>{ control },
            Err(str) => {panic!("err:{}",str)}
        };
        control.print_hello();
        println!("subscription-name:{}",control.subscription_name());
        println!("exist:{}",control.is_exist());
    }
}
