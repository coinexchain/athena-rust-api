use super::native;

pub fn println(msg: &str) {
    unsafe {
        native::sci_print(msg.as_ptr(), msg.len() as i32);
    }
}
