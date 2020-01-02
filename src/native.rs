extern "C" {
    pub fn sci_get_route_string(len: *mut i32) -> *mut u8;
    pub fn sci_get_caller(ptr: *mut u8);
    pub fn sci_get_address(ptr: *mut u8); // get 160 bits address
    pub fn sci_get_balance(ptr: *mut u8); // get 256 bits big int
    pub fn sci_transfer(to_addr: *const u8, amt: *const u8);
    pub fn sci_bigint_from_int64(n: i64, bytes: *mut u8, sign: *mut i8);

// TODO
}
