//! Functions related to KV Store.

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

// struct IterResult {
//     key_ptr: i32,
//     key_len: i32,
//     val_ptr: i32,
//     val_len: i32,
//     is_valid: i32,
// }

pub fn iterator_new(start: &[u8], end: &[u8]) -> native::Handle {
    unsafe { native::sci_kv_iterator(start.as_ptr(), start.len() as i32, end.as_ptr(), end.len() as i32) }
}
pub fn iterator_next(h: native::Handle) -> Option<(HostData, HostData)> {
    let mut r: [i32; 5] = [0; 5];
    unsafe {
        native::sci_kv_iterator_next(h, &mut r[0]);
        if native::is_ok(r[4]) {
            let key: HostData = std::slice::from_raw_parts(r[0] as *mut u8, r[1] as usize);
            let val: HostData = std::slice::from_raw_parts(r[2] as *mut u8, r[3] as usize);
            Some((key, val))
        } else {
            None
        }
    }
}
pub fn iterator_close(h: native::Handle) {
    unsafe {
        native::sci_kv_iterator_close(h);
    }
}

pub fn reverse_iterator_new(start: &[u8], end: &[u8]) -> native::Handle {
    unsafe { native::sci_kv_reverse_iterator(start.as_ptr(), start.len() as i32, end.as_ptr(), end.len() as i32) }
}
pub fn reverse_iterator_next(h: native::Handle) -> Option<(HostData, HostData)> {
    let mut r: [i32; 5] = [0; 5];
    unsafe {
        native::sci_kv_reverse_iterator_next(h, &mut r[0]);
        if native::is_ok(r[4]) {
            let key: HostData = std::slice::from_raw_parts(r[0] as *mut u8, r[1] as usize);
            let val: HostData = std::slice::from_raw_parts(r[2] as *mut u8, r[3] as usize);
            Some((key, val))
        } else {
            None
        }
    }
}
pub fn reverse_iterator_close(h: native::Handle) {
    unsafe {
        native::sci_kv_reverse_iterator_close(h);
    }
}

#[cfg(test)]
mod tests {
    use super::super::native::{I32Ptr, RawPtr};
    use super::super::{HostData, HostStr};
    use super::*;

    static mut VALUE: [u8; 5] = ['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8];

    #[no_mangle]
    pub extern "C" fn sci_kv_get(key: RawPtr, key_len: i32, value_len: I32Ptr) -> RawPtr {
        unsafe {
            if get_host_str(key, key_len) == "key" {
                *value_len = 5;
                VALUE.as_ptr()
            } else {
                0 as RawPtr
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn sci_kv_set(key: RawPtr, key_len: i32, val: RawPtr, val_len: i32) {
        if get_host_str(key, key_len) == "key" {
            let val = get_host_str(val, val_len);
            for i in 0..5 {
                if i < val.len() {
                    unsafe {
                        VALUE[i] = val.bytes().nth(i).unwrap();
                    }
                }
            }
        }
    }

    fn get_host_str(ptr: RawPtr, len: i32) -> HostStr {
        unsafe {
            let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
            let s: HostStr = std::str::from_utf8_unchecked(bytes);
            s
        }
    }

    #[test]
    fn api() {
        assert_eq!(get_str("foo"), None);
        assert_eq!(get_str("key"), Some("hello"));
        set_str("key", "world");
        assert_eq!(get_str("key"), Some("world"));
    }
}
