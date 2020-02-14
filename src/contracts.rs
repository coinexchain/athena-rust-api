use super::{native, HostData, HostStr};

/*
    pub fn sci_query_module(
        route: RawPtr,
        route_len: i32,
        param: RawPtr,
        param_len: i32,
        res_len_ptr: I32Ptr,
    ) -> RawPtr;
*/
pub fn query_module(route: &str, params: &[u8]) {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_query_module(
            route.as_ptr(),
            route.len() as i32,
            params.as_ptr(),
            params.len() as i32,
            &mut len,
        );
        // TODO
    }
}

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

pub fn prepare_coins(denom: &str, amt: i64) {
    unsafe { native::sci_prepare_coins(denom.as_ptr(), denom.len() as i32, amt) }
}

pub fn msg2run_append(msg_type: &[u8], msg_json: &[u8]) {
    unsafe {
        native::sci_messagestorun_append(
            msg_type.as_ptr(),
            msg_type.len() as i32,
            msg_json.as_ptr(),
            msg_json.len() as i32,
        )
    }
}
pub fn msg2run_len() -> i32 {
    unsafe { native::sci_messagestorun_len() }
}
pub fn msg2run_get(n: i32) {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_messagestorun_get(n, &mut len);
        // TODO
    }
}
