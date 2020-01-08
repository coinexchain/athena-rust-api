pub type int64_t = i64;
pub type int32_t = i32;
pub type bool_t = i32;
pub type mpint_t = i32;
pub type mpdec_t = i32;
pub type iter_t = i32;
pub type rev_iter_t = i32;
pub type int64_ptr_t = *mut i64;
pub type size_ptr_t = *mut i32;
pub type bool_ptr_t = *mut i32;
pub type ptr_t = *mut u8;
pub type ptr_t_ro = *const u8;
pub type iter_result_ptr_t = *mut u8;

pub fn is_ok(ret: i32) -> bool {
    ret > 0
}

extern "C" {
    pub fn sci_get_balance(denom_ptr: ptr_t_ro, denom_len: int32_t) -> mpint_t;
    pub fn sci_transfer(to_addr: ptr_t_ro, to_addr_len: int32_t, denom_ptr: ptr_t_ro, denom_len: int32_t, amt: mpint_t);

    // route & params
    pub fn sci_get_route_string(len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_param_decode_as_cbor() -> int32_t;
    pub fn sci_param_decode_as_json() -> int32_t;
    pub fn sci_param_count() -> int32_t;
    pub fn sci_get_param_addr(len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_param_to_int64(n: int32_t, ok: bool_ptr_t) -> int64_t;
    pub fn sci_param_to_int32(n: int32_t, ok: bool_ptr_t) -> int32_t;
    pub fn sci_param_to_string(n: int32_t, len_ptr: size_ptr_t, ok: bool_ptr_t) -> ptr_t;
    pub fn sci_param_to_byteslice(n: int32_t, len_ptr: size_ptr_t, ok: bool_ptr_t) -> ptr_t;
    pub fn sci_param_to_int64_array(n: int32_t, len_ptr: size_ptr_t, ok: bool_ptr_t) -> int64_ptr_t;

    // env
    pub fn sci_get_caller(len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_get_creator(len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_get_height() -> int64_t;
    pub fn sci_get_data_hash(len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_get_timestamp(sec_ptr: int64_ptr_t, nanosec_ptr: int64_ptr_t);
    pub fn sci_get_gas_limit() -> int64_t;
    pub fn sci_get_gas_remained() -> int64_t;

    // KV store
    pub fn sci_kv_get(key: ptr_t_ro, key_len: int32_t, value_len: size_ptr_t) -> ptr_t;
    pub fn sci_kv_set(key: ptr_t_ro, key_len: int32_t, value: ptr_t_ro, value_len: int32_t);
    pub fn sci_kv_erase(key: ptr_t_ro, key_len: int32_t);
    pub fn sci_kv_iterator(start: ptr_t, start_len: int32_t, end: ptr_t, end_len: int32_t) -> iter_t;
    pub fn sci_kv_iterator_next(iter: iter_t, res_ptr: iter_result_ptr_t);
    pub fn sci_kv_iterator_close(iter: iter_t);
    pub fn sci_kv_reverse_iterator(start: ptr_t, start_len: int32_t, end: ptr_t, end_len: int32_t) -> rev_iter_t;
    pub fn sci_kv_reverse_iterator_next(rev_iter: iter_t, res_ptr: iter_result_ptr_t);
    pub fn sci_kv_reverse_iterator_close(rev_iter: iter_t);

    // events
    pub fn sci_event_begin(evt_type: ptr_t_ro, evt_type_len: int32_t);
    pub fn sci_event_add_attribute(key: ptr_t_ro, key_len: int32_t, value: ptr_t_ro, value_len: int32_t);
    pub fn sci_event_end();

    // big int
    pub fn sci_mpint_allocate() -> mpint_t;
    pub fn sci_mpint_free(i: mpint_t);
    pub fn sci_mpint_to_string(i: mpint_t, len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_mpint_from_string(i: mpint_t, str_ptr: ptr_t_ro, str_len: int32_t) -> int32_t;
    pub fn sci_mpint_from_int64(i: mpint_t, val: int64_t) -> int32_t;
    pub fn sci_mpint_add(z: mpint_t, a: mpint_t, b: mpint_t);
    pub fn sci_mpint_sub(z: mpint_t, a: mpint_t, b: mpint_t);
    pub fn sci_mpint_mul(z: mpint_t, a: mpint_t, b: mpint_t);
    pub fn sci_mpint_eq(a: mpint_t, b: mpint_t) -> bool_t;
    pub fn sci_mpint_gt(a: mpint_t, b: mpint_t) -> bool_t;
    pub fn sci_mpint_gte(a: mpint_t, b: mpint_t) -> bool_t;
    pub fn sci_mpint_lt(a: mpint_t, b: mpint_t) -> bool_t;
    pub fn sci_mpint_lte(a: mpint_t, b: mpint_t) -> bool_t;

    // big dec
    pub fn sci_mpdec_allocate() -> mpdec_t;
    pub fn sci_mpdec_free(i: mpdec_t);
    pub fn sci_mpdec_to_string(i: mpdec_t, len_ptr: size_ptr_t) -> ptr_t;
    pub fn sci_mpdec_from_string(i: mpdec_t, str_ptr: ptr_t, str_len: int32_t) -> int32_t;
    pub fn sci_mpdec_add(z: mpdec_t, a: mpdec_t, b: mpdec_t);
    pub fn sci_mpdec_sub(z: mpdec_t, a: mpdec_t, b: mpdec_t);
    pub fn sci_mpdec_mul(z: mpdec_t, a: mpdec_t, b: mpdec_t);
}
