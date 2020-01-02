mod big_int;
pub mod kv;
pub mod native;

pub use big_int::BigInt;

pub type Address = Vec<u8>; // AccAddress

pub fn get_route() -> String {
    unsafe {
        let mut len: i32 = 0;
        let route_raw = native::sci_get_route_string(&mut len);
        String::from_raw_parts(route_raw, len as usize, len as usize)
    }
}

pub fn get_caller() -> Vec<u8> {
    unsafe {
        let mut addr_len: i32 = 0;
        let addr_raw = native::sci_get_caller(&mut addr_len);
        Vec::from_raw_parts(addr_raw, addr_len as usize, addr_len as usize)
    }
}

pub fn get_creator() -> Vec<u8> {
    unsafe {
        let mut addr_len: i32 = 0;
        let addr_raw = native::sci_get_creator(&mut addr_len);
        Vec::from_raw_parts(addr_raw, addr_len as usize, addr_len as usize)
    }
}

pub fn get_balance() -> BigInt {
    unsafe { BigInt::wrap(native::sci_get_balance()) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
