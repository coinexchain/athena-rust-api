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

    pub fn zero() -> BigInt {
        BigInt::from_i64(0)
    }

    pub fn from_i64(n: i64) -> BigInt {
        let bi = BigInt::new();
        bi.set_i64(n);
        bi
    }

    pub fn from_str(s: &str) -> Option<BigInt> {
        BigInt::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(s: &[u8]) -> Option<BigInt> {
        unsafe {
            let bi = BigInt::new();
            let ok = native::sci_mpint_from_string(bi.handle, s.as_ptr(), s.len() as i32);
            if native::is_ok(ok) {
                Some(bi)
            } else {
                None
            }
        }
    }

    pub fn new() -> BigInt {
        unsafe {
            BigInt {
                handle: native::sci_mpint_allocate(),
            }
        }
    }

    // pub fn add(z: &BigInt, a: &BigInt, b: &BigInt) {
    //     unsafe { native::sci_mpint_add(z.handle, a.handle, b.handle) }
    // }
    // pub fn sub(z: &BigInt, a: &BigInt, b: &BigInt) {
    //     unsafe { native::sci_mpint_sub(z.handle, a.handle, b.handle) }
    // }
    // pub fn mul(z: &BigInt, a: &BigInt, b: &BigInt) {
    //     unsafe { native::sci_mpint_mul(z.handle, a.handle, b.handle) }
    // }

    pub fn add(&self, b: &BigInt) -> BigInt {
        unsafe {
            let c = BigInt::new();
            native::sci_mpint_add(c.handle, self.handle, b.handle);
            c
        }
    }

    pub fn sub(&self, b: &BigInt) -> BigInt {
        unsafe {
            let c = BigInt::new();
            native::sci_mpint_sub(c.handle, self.handle, b.handle);
            c
        }
    }

    pub fn mul(&self, b: &BigInt) -> BigInt {
        unsafe {
            let c = BigInt::new();
            native::sci_mpint_mul(c.handle, self.handle, b.handle);
            c
        }
    }

    pub fn eq(&self, b: &BigInt) -> bool {
        unsafe {
            let ok = native::sci_mpint_eq(self.handle, b.handle);
            native::is_ok(ok)
        }
    }

    pub fn gt(&self, b: &BigInt) -> bool {
        unsafe {
            let ok = native::sci_mpint_gt(self.handle, b.handle);
            native::is_ok(ok)
        }
    }

    pub fn gte(&self, b: &BigInt) -> bool {
        unsafe {
            let ok = native::sci_mpint_gte(self.handle, b.handle);
            native::is_ok(ok)
        }
    }

    pub fn lt(&self, b: &BigInt) -> bool {
        unsafe {
            let ok = native::sci_mpint_lt(self.handle, b.handle);
            native::is_ok(ok)
        }
    }

    pub fn lte(&self, b: &BigInt) -> bool {
        unsafe {
            let ok = native::sci_mpint_lte(self.handle, b.handle);
            native::is_ok(ok)
        }
    }

    pub fn set_i64(&self, n: i64) -> bool {
        unsafe {
            let ok = native::sci_mpint_from_int64(self.handle, n);
            native::is_ok(ok)
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let mut str_len = 0i32;
            let str_raw = native::sci_mpint_to_string(self.handle, &mut str_len);
            if str_raw as i32 == 0 {
                String::from("???") // TODO
            } else {
                String::from_raw_parts(str_raw, str_len as usize, str_len as usize)
            }
        }
    }

    pub fn get_handle(&self) -> native::mpint_t {
        self.handle
    }
}
