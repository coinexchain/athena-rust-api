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

pub fn get_i32(idx: i32) -> Option<i32> {
    unsafe {
        let mut ok = 0i32;
        let val = native::sci_param_to_int32(idx, &mut ok);
        if native::is_ok(ok) {
            Some(val)
        } else {
            None
        }
    }
}

pub fn get_i64(idx: i32) -> Option<i64> {
    unsafe {
        let mut ok = 0i32;
        let val = native::sci_param_to_int64(idx, &mut ok);
        if native::is_ok(ok) {
            Some(val)
        } else {
            None
        }
    }
}

pub fn get_string(idx: i32) -> Option<String> {
    unsafe {
        let mut len = 0i32;
        let mut ok = 0i32;
        let str_raw = native::sci_param_to_string(idx, &mut len, &mut ok);
        if native::is_ok(ok) {
            Some(String::from_raw_parts(str_raw, len as usize, len as usize))
        } else {
            None
        }
    }
}
