#![allow(dead_code)]

#[macro_export]
macro_rules! debug_log {
    ($system:expr, $message:expr) => {
        println!("[DEBUG] {:?} : {:?}", $system, $message);
    };
}
