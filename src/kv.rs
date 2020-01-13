use super::native;

pub fn get(key: &[u8]) -> Option<&'static [u8]> {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_kv_get(key.as_ptr(), key.len() as i32, &mut len);
        if ptr as usize > 0 {
            let bytes: &'static [u8] = std::slice::from_raw_parts(ptr, len as usize);
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
