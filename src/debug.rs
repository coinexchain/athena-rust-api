use super::native;

pub fn println(msg: &str) {
    unsafe {
        native::sci_print_str(msg.as_ptr(), msg.len() as i32);
    }
}

pub fn print_i32(i: i32) {
    unsafe {
        native::sci_print_int32(i);
    }
}

pub fn print_i64(i: i64) {
    unsafe {
        native::sci_print_int64(i);
    }
}
