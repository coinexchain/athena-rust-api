//! Functions related to ABCI Events.

use super::{native, HostData, HostStr};

pub fn emit(evt_type: &str, attrs: &[(&str, &str)]) {
    begin(evt_type);
    for kv in attrs {
        add_attr(kv.0, kv.1);
    }
    end();
}

pub fn begin(evt_type: &str) {
    unsafe {
        native::sci_event_begin(evt_type.as_ptr(), evt_type.len() as i32);
    }
}
pub fn add_attr(k: &str, v: &str) {
    unsafe {
        native::sci_event_add_attribute(k.as_ptr(), k.len() as i32, v.as_ptr(), v.len() as i32);
    }
}
pub fn end() {
    unsafe {
        native::sci_event_end();
    }
}

pub fn count() -> i32 {
    unsafe { native::sci_event_count() }
}

pub fn get_type(id: i32) -> HostStr {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_event_get_type(id, &mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        let s: HostStr = std::str::from_utf8_unchecked(bytes);
        s
    }
}
pub fn get_attr(id: i32, k: &str) -> HostStr {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_event_get_attribute(id, k.as_ptr(), k.len() as i32, &mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        let s: HostStr = std::str::from_utf8_unchecked(bytes);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::super::native::{I32Ptr, RawPtr};
    use super::*;

    static mut BUF: [u8; 1024] = [0; 1024];
    static mut POS: usize = 0;

    fn copy_data(ptr: RawPtr, len: i32) {
        unsafe {
            for i in 0..len {
                BUF[POS] = *((ptr as isize + i as isize) as *const u8);
                POS += 1;
            }
        }
    }
    fn copy_str(s: &str) {
        copy_data(s.as_ptr(), s.len() as i32)
    }
    fn get_buf(len: usize) -> &'static [u8] {
        unsafe { &BUF[0..len] }
    }

    #[no_mangle]
    pub fn sci_event_begin(ptr: RawPtr, len: i32) {
        copy_data(ptr, len);
        copy_str("{");
    }
    #[no_mangle]
    pub fn sci_event_add_attribute(key_ptr: RawPtr, key_len: i32, val_ptr: RawPtr, val_len: i32) {
        copy_data(key_ptr, key_len);
        copy_str(":");
        copy_data(val_ptr, val_len);
        copy_str(",");
    }
    #[no_mangle]
    pub fn sci_event_end() {
        copy_str("}");
    }
    #[no_mangle]
    pub fn sci_event_count() -> i32 {
        7
    }
    #[no_mangle]
    pub fn sci_event_get_type(id: i32, len_ptr: I32Ptr) -> RawPtr {
        assert_eq!(id, 6);
        unsafe {
            *len_ptr = 6;
            "event6".as_ptr()
        }
    }
    #[no_mangle]
    pub fn sci_event_get_attribute(id: i32, key: RawPtr, key_len: i32, len_ptr: I32Ptr) -> RawPtr {
        assert_eq!(id, 6);
        unsafe {
            *len_ptr = 4;
            "attr".as_ptr()
        }
    }

    #[test]
    fn api() {
        emit("evt", &[("foo", "bar"), ("key", "val")]);
        assert_eq!(count(), 7);
        assert_eq!(get_type(6), "event6");
        assert_eq!(get_attr(6, "key"), "attr");
        assert_eq!(get_buf(21), "evt{foo:bar,key:val,}".as_bytes());
    }
}
