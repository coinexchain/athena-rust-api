use super::native;

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
