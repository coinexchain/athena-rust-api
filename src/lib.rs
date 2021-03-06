mod big_int;
mod debug;

pub mod block;
pub mod contracts;
pub mod crypto;
pub mod events;
pub mod kv;
pub mod macros;
pub mod map;
pub mod native;
pub mod params;

pub use big_int::BigInt;
pub use debug::println;
pub use params::get_route;

pub type HostData = &'static [u8];
pub type HostStr = &'static str;

// pub type Address<'a> = &'a [u8]; // AccAddress

pub fn get_caller_bech32() -> HostStr {
    let addr = get_caller();
    addr_to_bech32(addr).unwrap()
}

pub fn get_caller() -> HostData {
    unsafe {
        let ptr = native::sci_get_caller();
        let bytes: HostData = std::slice::from_raw_parts(ptr, native::ADDR_LEN);
        bytes
    }
}

pub fn get_creator() -> HostData {
    unsafe {
        let ptr = native::sci_get_creator();
        let bytes: HostData = std::slice::from_raw_parts(ptr, native::ADDR_LEN);
        bytes
    }
}

pub fn get_gas_limit() -> i64 {
    unsafe { native::sci_get_gas_limit() }
}
pub fn get_gas_remained() -> i64 {
    unsafe { native::sci_get_gas_remained() }
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

pub fn addr_from_bech32(s: &str) -> Option<HostData> {
    unsafe {
        let ptr = native::sci_address_from_bech32(s.as_ptr(), s.len() as i32);
        if ptr as i32 > 0 {
            Some(std::slice::from_raw_parts(ptr, native::ADDR_LEN))
        } else {
            None
        }
    }
}
pub fn addr_to_bech32(addr: HostData) -> Option<HostStr> {
    unsafe {
        let mut len = 0i32;
        let ptr = native::sci_address_to_bech32(addr.as_ptr(), &mut len);
        if ptr as i32 > 0 {
            let bytes = std::slice::from_raw_parts(ptr, len as usize);
            let s: HostStr = std::str::from_utf8_unchecked(bytes);
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
