use super::native;

pub fn publish(evt_type: &str, attrs: &Vec<(&str, &str)>) {
    unsafe {
        native::sci_event_begin(evt_type.as_ptr(), evt_type.len() as i32);

        for kv in attrs {
            let k = kv.0;
            let v = kv.1;
            native::sci_event_add_attribute(k.as_ptr(), k.len() as i32, v.as_ptr(), v.len() as i32);
        }

        native::sci_event_end();
    }
}
