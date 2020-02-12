//! Debug functions.

use super::native;

pub fn print(msg: &str) {
    unsafe {
        native::sci_print(msg.as_ptr(), msg.len() as i32);
    }
}
pub fn println(msg: &str) {
    unsafe {
        native::sci_println(msg.as_ptr(), msg.len() as i32);
    }
}

pub fn print_i64(i: i64) {
    unsafe {
        native::sci_print_int64(i);
    }
}
