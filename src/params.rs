//! Functions related to Route & Params.

use super::{native, HostData, HostStr};

pub fn get_route() -> HostStr {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_get_route_string(&mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        let s: HostStr = std::str::from_utf8_unchecked(bytes);
        s
    }
}

pub fn decode_as_json() -> bool {
    unsafe {
        let ok = native::sci_param_decode_as_json();
        native::is_ok(ok)
    }
}

pub fn decode_as_cbor() -> bool {
    unsafe {
        let ok = native::sci_param_decode_as_cbor();
        native::is_ok(ok)
    }
}

pub fn count() -> i32 {
    unsafe { native::sci_param_count() }
}

pub fn get_i32(idx: i32) -> i32 {
    unsafe { native::sci_param_to_int32(idx) }
}

pub fn get_i64(idx: i32) -> i64 {
    unsafe { native::sci_param_to_int64(idx) }
}

pub fn get_str(idx: i32) -> HostStr {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_param_to_string(idx, &mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        let s: HostStr = std::str::from_utf8_unchecked(bytes);
        s
    }
}

pub fn get_bytes(idx: i32) -> HostData {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_param_to_byteslice(idx, &mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        bytes
    }
}

// pub fn sci_param_to_int64_array(n: i32, len_ptr: I32Ptr) -> I64Ptr;
// pub fn sci_get_param_addr(len_ptr: I32Ptr) -> RawPtr;
// pub fn sci_get_paramraw_addr(len_ptr: I32Ptr) -> i32;

#[cfg(test)]
mod tests {
    use super::super::native::{I32Ptr, RawPtr};
    use super::*;

    #[no_mangle]
    pub extern "C" fn sci_get_route_string(len_ptr: I32Ptr) -> RawPtr {
        unsafe {
            *len_ptr = 3;
            "foo".as_ptr()
        }
    }
    #[no_mangle]
    pub extern "C" fn sci_param_count() -> i32 {
        5
    }
    #[no_mangle]
    pub fn sci_param_to_int32(n: i32) -> i32 {
        assert_eq!(n, 0);
        300
    }
    #[no_mangle]
    pub fn sci_param_to_int64(n: i32) -> i64 {
        assert_eq!(n, 1);
        600
    }
    #[no_mangle]
    pub fn sci_param_to_string(n: i32, len_ptr: I32Ptr) -> RawPtr {
        assert_eq!(n, 2);
        unsafe {
            *len_ptr = 3;
            "bar".as_ptr()
        }
    }
    #[no_mangle]
    pub fn sci_param_to_byteslice(n: i32, len_ptr: I32Ptr) -> RawPtr {
        assert_eq!(n, 3);
        unsafe {
            *len_ptr = 4;
            "data".as_ptr()
        }
    }

    #[test]
    fn api() {
        assert_eq!(get_route(), "foo");
        assert_eq!(count(), 5);
        assert_eq!(get_i32(0), 300);
        assert_eq!(get_i64(1), 600);
        assert_eq!(get_str(2), "bar");
        assert_eq!(get_bytes(3), "data".as_bytes());
    }
}
