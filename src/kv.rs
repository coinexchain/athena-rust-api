use super::native;

pub fn get(key: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let mut val_len = 0i32;
        let val_raw = native::sci_kv_get(key.as_ptr(), key.len() as i32, &mut val_len);
        if val_raw as usize > 0 {
            let val = Vec::from_raw_parts(val_raw, val_len as usize, val_len as usize);
            Some(val)
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
