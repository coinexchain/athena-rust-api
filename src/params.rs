use super::native;

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

pub fn get_string(idx: i32) -> &'static str {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_param_to_string(idx, &mut len);
        let bytes: &'static [u8] = std::slice::from_raw_parts(ptr, len as usize);
        let s: &'static str = std::str::from_utf8_unchecked(bytes);
        s
    }
}

pub fn get_bytes(idx: i32) -> &'static [u8] {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_param_to_byteslice(idx, &mut len);
        let bytes: &'static [u8] = std::slice::from_raw_parts(ptr, len as usize);
        bytes
    }
}
