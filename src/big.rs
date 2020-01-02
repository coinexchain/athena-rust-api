use super::native;

pub struct Int256 {
    pub bytes: [u8; 32],
    pub sign: i8, // positive: >0, negative: <0
}

pub fn int_from_i64(n: i64) -> Int256 {
    unsafe {
        let mut bytes = [0u8; 32];
        let mut sign = 0i8;
        native::sci_bigint_from_int64(n, bytes.as_mut_ptr(), &mut sign);
        Int256 {
            bytes: bytes,
            sign: sign,
        }
    }
}
