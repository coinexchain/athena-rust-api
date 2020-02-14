use super::{native, HostData, HostStr};

/*
    pub fn sci_call_contract(
        id: i64,
        route: RawPtr,
        route_len: i32,
        param: RawPtr,
        param_len: i32,
        queryRetPtr: I32Ptr,
        queryLenPtr: I32Ptr,
    );
*/
pub fn call(id: i64, route: &str, params: &[u8]) {
    unsafe {
        let mut ret: i32 = 0;
        let mut len: i32 = 0;
        native::sci_call_contract(
            id,
            route.as_ptr(),
            route.len() as i32,
            params.as_ptr(),
            params.len() as i32,
            &mut ret,
            &mut len,
        );
        // TODO
    }
}