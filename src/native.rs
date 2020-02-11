pub type Bool = i32;
pub type Handle = i32;
pub type I64Ptr = *mut i64;
pub type I32Ptr = *mut i32;
pub type RawPtr = *mut u8;
pub type RawPtrRO = *const u8;

pub const ADDR_LEN: usize = 20;

pub fn is_ok(ret: i32) -> bool {
    ret > 0
}

extern "C" {
    pub fn sci_get_balance(denom_ptr: RawPtrRO, denom_len: i32) -> Handle;
    pub fn sci_transfer(to_addr: RawPtrRO, denom_ptr: RawPtrRO, denom_len: i32, amt: Handle);

    // address
    pub fn sci_address_from_bech32(bech32_ptr: RawPtrRO, bech32_len: i32) -> RawPtr;
    pub fn sci_address_to_bech32(addr_ptr: RawPtrRO, bech32_len_ptr: I32Ptr) -> RawPtr;

    // route & params
    pub fn sci_get_route_string(len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_param_decode_as_cbor() -> i32;
    pub fn sci_param_decode_as_json() -> i32;
    pub fn sci_param_count() -> i32;
    pub fn sci_get_param_addr(len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_param_to_int64(n: i32) -> i64;
    pub fn sci_param_to_int32(n: i32) -> i32;
    pub fn sci_param_to_string(n: i32, len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_param_to_byteslice(n: i32, len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_param_to_int64_array(n: i32, len_ptr: I32Ptr) -> I64Ptr;

    // env
    pub fn sci_get_caller() -> RawPtr;
    pub fn sci_get_creator() -> RawPtr;
    pub fn sci_get_height() -> i64;
    pub fn sci_get_data_hash(len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_get_timestamp(sec_ptr: I64Ptr, nanosec_ptr: I64Ptr);
    // pub fn sci_get_gas_limit() -> i64;
    // pub fn sci_get_gas_remained() -> i64;

    // KV store
    pub fn sci_kv_get(key: RawPtrRO, key_len: i32, value_len: I32Ptr) -> RawPtrRO;
    pub fn sci_kv_set(key: RawPtrRO, key_len: i32, value: RawPtrRO, value_len: i32);
    pub fn sci_kv_erase(key: RawPtrRO, key_len: i32);
    pub fn sci_kv_iterator(start: RawPtr, start_len: i32, end: RawPtr, end_len: i32) -> Handle;
    pub fn sci_kv_iterator_next(iter: Handle, res_ptr: RawPtr);
    pub fn sci_kv_iterator_close(iter: Handle);
    pub fn sci_kv_reverse_iterator(start: RawPtr, start_len: i32, end: RawPtr, end_len: i32) -> Handle;
    pub fn sci_kv_reverse_iterator_next(rev_iter: Handle, res_ptr: RawPtr);
    pub fn sci_kv_reverse_iterator_close(rev_iter: Handle);

    // events
    pub fn sci_event_begin(evt_type: RawPtrRO, evt_type_len: i32);
    pub fn sci_event_add_attribute(key: RawPtrRO, key_len: i32, value: RawPtrRO, value_len: i32);
    pub fn sci_event_end();

    // big int
    pub fn sci_mpint_allocate() -> Handle;
    pub fn sci_mpint_free(i: Handle);
    pub fn sci_mpint_to_string(i: Handle, len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_mpint_from_string(i: Handle, str_ptr: RawPtrRO, str_len: i32);
    pub fn sci_mpint_from_int64(i: Handle, val: i64);
    pub fn sci_mpint_add(z: Handle, a: Handle, b: Handle);
    pub fn sci_mpint_sub(z: Handle, a: Handle, b: Handle);
    pub fn sci_mpint_mul(z: Handle, a: Handle, b: Handle);
    pub fn sci_mpint_eq(a: Handle, b: Handle) -> Bool;
    pub fn sci_mpint_gt(a: Handle, b: Handle) -> Bool;
    pub fn sci_mpint_gte(a: Handle, b: Handle) -> Bool;
    pub fn sci_mpint_lt(a: Handle, b: Handle) -> Bool;
    pub fn sci_mpint_lte(a: Handle, b: Handle) -> Bool;

    // big dec
    pub fn sci_mpdec_allocate() -> Handle;
    pub fn sci_mpdec_free(i: Handle);
    pub fn sci_mpdec_to_string(i: Handle, len_ptr: I32Ptr) -> RawPtr;
    pub fn sci_mpdec_from_string(i: Handle, str_ptr: RawPtr, str_len: i32);
    pub fn sci_mpdec_add(z: Handle, a: Handle, b: Handle);
    pub fn sci_mpdec_sub(z: Handle, a: Handle, b: Handle);
    pub fn sci_mpdec_mul(z: Handle, a: Handle, b: Handle);

    // crypto
    pub fn sci_verifysig(addr: RawPtrRO, digest: RawPtrRO, digest_len: i32, sig: RawPtrRO, sig_len: i32) -> i32;
    pub fn sci_sha256(data: RawPtrRO, data_len: i32) -> RawPtr;
    pub fn sci_sha256_begin();
    pub fn sci_sha256_write(data: i32, dataLen: i32);
    pub fn sci_sha256_sum() -> RawPtr;

    // debug
    pub fn sci_print(str_ptr: RawPtrRO, str_len: i32);
    pub fn sci_println(str_ptr: RawPtrRO, str_len: i32);
    pub fn sci_print_int64(i: i64);
}
