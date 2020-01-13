mod big_int;
mod debug;
pub mod events;
pub mod kv;
pub mod macros;
pub mod native;
pub mod params;

pub use big_int::BigInt;
pub use debug::println;

// pub type Address = &'[u8]; // AccAddress

pub type HostData = &'static [u8];
pub type HostStr = &'static str;

pub fn get_route() -> HostStr {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_get_route_string(&mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        let s: HostStr = std::str::from_utf8_unchecked(bytes);
        s
    }
}

pub fn get_caller() -> HostData {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_get_caller(&mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        bytes
    }
}

pub fn get_creator() -> HostData {
    unsafe {
        let mut len: i32 = 0;
        let ptr = native::sci_get_creator(&mut len);
        let bytes: HostData = std::slice::from_raw_parts(ptr, len as usize);
        bytes
    }
}

pub fn get_balance() -> BigInt {
    unsafe {
        let cet = "cet";
        BigInt::wrap(native::sci_get_balance(cet.as_ptr(), cet.len() as i32))
    }
}

pub fn transfer(to_addr: &[u8], amt: &BigInt) {
    unsafe {
        let cet = "cet";
        native::sci_transfer(to_addr.as_ptr(), cet.as_ptr(), cet.len() as i32, amt.get_handle());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
