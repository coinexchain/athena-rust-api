use super::native;

pub struct BigInt {
    handle: native::mpint_t,
}

impl Drop for BigInt {
    fn drop(&mut self) {
        unsafe { native::sci_mpint_free(self.handle) }
    }
}

impl BigInt {
    pub fn wrap(handle: native::mpint_t) -> BigInt {
        BigInt { handle: handle }
    }

    pub fn from_i64(n: i64) -> BigInt {
        let bi = BigInt::new();
        bi.set_i64(n);
        bi
    }

    pub fn new() -> BigInt {
        unsafe {
            BigInt {
                handle: native::sci_mpint_allocate(),
            }
        }
    }

    pub fn add(z: &BigInt, a: &BigInt, b: &BigInt) {
        unsafe { native::sci_mpint_add(z.handle, a.handle, b.handle) }
    }

    pub fn sub(z: &BigInt, a: &BigInt, b: &BigInt) {
        unsafe { native::sci_mpint_sub(z.handle, a.handle, b.handle) }
    }

    pub fn mul(z: &BigInt, a: &BigInt, b: &BigInt) {
        unsafe { native::sci_mpint_mul(z.handle, a.handle, b.handle) }
    }

    pub fn set_i64(&self, n: i64) -> bool {
        unsafe {
            let ok = native::sci_mpint_from_int64(self.handle, n);
            native::is_ok(ok)
        }
    }
}
