use super::{native, HostData, HostStr};

pub fn get_str(key: &str) -> Option<HostStr> {
    let bytes = get(key.as_bytes());
    if bytes.is_some() {
        unsafe { Some(std::str::from_utf8_unchecked(bytes.unwrap())) }
    } else {
        None
    }
}

pub fn set_str(key: &str, val: &str) {
    set(key.as_bytes(), val.as_bytes());
}

pub fn del_str(key: &str) {
    del(key.as_bytes());
}

pub fn get(key: &[u8]) -> Option<HostData> {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_kv_get(key.as_ptr(), key.len() as i32, &mut len);
        if ptr as i32 > 0 {
            let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
            Some(bytes)
        } else {
            None
        }
    }
}

pub fn set(key: &[u8], val: &[u8]) {
    unsafe {
        native::sci_kv_set(key.as_ptr(), key.len() as i32, val.as_ptr(), val.len() as i32);
    }
}

pub fn del(key: &[u8]) {
    unsafe {
        native::sci_kv_erase(key.as_ptr(), key.len() as i32);
    }
}
