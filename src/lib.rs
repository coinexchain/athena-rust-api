mod big_int;
mod debug;
pub mod events;
pub mod kv;
pub mod macros;
pub mod native;
pub mod params;

pub use big_int::BigInt;
pub use debug::println;

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
    unsafe {
        let cet = "cet";
        BigInt::wrap(native::sci_get_balance(cet.as_ptr(), cet.len() as i32))
    }
}

pub fn transfer(to_addr: &Vec<u8>, amt: &BigInt) {
    unsafe {
        let cet = "cet";
        native::sci_transfer(
            to_addr.as_ptr(),
            to_addr.len() as i32,
            cet.as_ptr(),
            cet.len() as i32,
            amt.get_handle(),
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
