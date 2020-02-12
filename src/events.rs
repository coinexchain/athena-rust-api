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

pub fn get_type(id: i32) -> HostData {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_event_get_type(id, &mut len);
        let data: HostData = std::slice::from_raw_parts(ptr, len as usize);
        data
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
