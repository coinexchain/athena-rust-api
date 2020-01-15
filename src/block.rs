use super::{native, HostData};

pub fn get_height() -> i64 {
    unsafe { native::sci_get_height() }
}

pub fn get_data_hash() -> HostData {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_get_data_hash(&mut len);
        std::slice::from_raw_parts(ptr, len as usize)
    }
}

pub fn get_timestamp() -> (i64, i64) {
    unsafe {
        let mut sec = 0i64;
        let mut nano = 0i64;
        native::sci_get_timestamp(&mut sec, &mut nano);
        (sec, nano)
    }
}
