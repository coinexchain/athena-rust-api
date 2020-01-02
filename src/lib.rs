pub mod big;
pub mod native;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bytes20 {
    pub bytes: [u8; 20],
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bytes32 {
    pub bytes: [u8; 32],
}

pub type Address = Bytes20; // AccAddress

pub fn get_route() -> String {
    unsafe {
        let mut len: i32 = 0;
        let route_raw = native::sci_get_route_string(&mut len);
        String::from_raw_parts(route_raw, len as usize, len as usize)
    }
}

pub fn get_caller() -> Address {
    unsafe {
        let mut bytes = [0u8; 20];
        native::sci_get_caller(bytes.as_mut_ptr());
        Address { bytes: bytes }
    }
}

pub fn get_address() -> Address {
    unsafe {
        let mut bytes = [0u8; 20];
        native::sci_get_address(bytes.as_mut_ptr());
        Address { bytes: bytes }
    }
}

pub fn get_balance() -> big::Int256 {
    unsafe {
        let mut bytes = [0u8; 32];
        native::sci_get_balance(bytes.as_mut_ptr());
        big::Int256 {
            bytes: bytes,
            sign: 1,
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
