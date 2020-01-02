use super::native;

pub fn get(key: Vec<u8>) -> Vec<u8> {
    unsafe {
        let mut val_len = 0i32;
        let val_raw = native::sci_kv_get(key.as_ptr(), key.len() as i32, &mut val_len);
        let val = Vec::from_raw_parts(val_raw, val_len as usize, val_len as usize);
        val
    }
}

pub fn set(key: Vec<u8>, val: Vec<u8>) {
    unsafe {
        native::sci_kv_set(key.as_ptr(), key.len() as i32, val.as_ptr(), val.len() as i32);
    }
}

pub fn del(key: Vec<u8>) {
    unsafe {
        native::sci_kv_erase(key.as_ptr(), key.len() as i32);
    }
}