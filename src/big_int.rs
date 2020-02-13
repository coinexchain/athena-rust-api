//! Big Integer.

use super::{native, HostStr};

pub struct BigInt {
    handle: native::Handle,
}

impl Drop for BigInt {
    fn drop(&mut self) {
        unsafe { native::sci_mpint_free(self.handle) }
    }
}

impl BigInt {
    pub fn wrap(handle: native::Handle) -> BigInt {
        BigInt { handle: handle }
    }

    pub fn zero() -> BigInt {
        BigInt::from_i64(0)
    }
    pub fn one() -> BigInt {
        BigInt::from_i64(1)
    }

    pub fn from_i64(n: i64) -> BigInt {
        let bi = BigInt::new();
        bi.set_i64(n);
        bi
    }

    pub fn from_str(s: &str) -> BigInt {
        BigInt::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(s: &[u8]) -> BigInt {
        unsafe {
            let bi = BigInt::new();
            native::sci_mpint_from_string(bi.handle, s.as_ptr(), s.len() as i32);
            bi
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

    pub fn set_i64(&self, n: i64) {
        unsafe { native::sci_mpint_from_int64(self.handle, n) }
    }

    pub fn to_str(&self) -> HostStr {
        unsafe {
            let mut len = 0i32;
            let ptr = native::sci_mpint_to_string(self.handle, &mut len);
            let bytes = std::slice::from_raw_parts(ptr, len as usize);
            let s: HostStr = std::str::from_utf8_unchecked(bytes);
            s
        }
    }

    pub fn get_handle(&self) -> native::Handle {
        self.handle
    }
}

#[cfg(test)]
mod tests {
    use super::super::native::{Bool, Handle, I32Ptr, RawPtr};
    use super::*;

    static mut POOL: [i64; 100] = [0; 100];
    static mut IDX: i32 = 0;

    fn get_val(x: &BigInt) -> i64 {
        unsafe { POOL[x.handle as usize] }
    }

    #[no_mangle]
    pub extern "C" fn sci_mpint_allocate() -> Handle {
        unsafe {
            IDX += 1;
            IDX - 1
        }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_free(i: Handle) {
        // do nothing
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_to_string(i: Handle, len_ptr: I32Ptr) -> RawPtr {
        panic!("TODO");
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_from_string(i: Handle, str_ptr: RawPtr, str_len: i32) {
        panic!("TODO");
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_from_int64(i: Handle, val: i64) {
        unsafe { POOL[i as usize] = val }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_add(z: Handle, a: Handle, b: Handle) {
        unsafe { POOL[z as usize] = POOL[a as usize] + POOL[b as usize] }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_sub(z: Handle, a: Handle, b: Handle) {
        unsafe { POOL[z as usize] = POOL[a as usize] - POOL[b as usize] }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_mul(z: Handle, a: Handle, b: Handle) {
        unsafe { POOL[z as usize] = POOL[a as usize] * POOL[b as usize] }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_eq(a: Handle, b: Handle) -> Bool {
        unsafe { toBool(POOL[a as usize] == POOL[b as usize]) }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_gt(a: Handle, b: Handle) -> Bool {
        unsafe { toBool(POOL[a as usize] > POOL[b as usize]) }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_gte(a: Handle, b: Handle) -> Bool {
        unsafe { toBool(POOL[a as usize] >= POOL[b as usize]) }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_lt(a: Handle, b: Handle) -> Bool {
        unsafe { toBool(POOL[a as usize] < POOL[b as usize]) }
    }
    #[no_mangle]
    pub extern "C" fn sci_mpint_lte(a: Handle, b: Handle) -> Bool {
        unsafe { toBool(POOL[a as usize] <= POOL[b as usize]) }
    }

    fn toBool(b: bool) -> Bool {
        if b {
            1
        } else {
            0
        }
    }

    #[test]
    fn api() {
        let a = BigInt::zero(); // 0
        let b = BigInt::one(); // 1
        let c = BigInt::from_i64(2); // 2
        let d = b.add(&c); // 3
        let e = c.mul(&d); // 6
        let f = e.sub(&b); // 5
        let g = BigInt::from_i64(5); // 5

        assert_eq!(get_val(&a), 0);
        assert_eq!(get_val(&b), 1);
        assert_eq!(get_val(&c), 2);
        assert_eq!(get_val(&d), 3);
        assert_eq!(get_val(&e), 6);
        assert_eq!(get_val(&f), 5);
        assert!(f.eq(&g));
        assert!(!f.eq(&e));
        assert!(f.gt(&d));
        assert!(f.lt(&e));
    }
}
