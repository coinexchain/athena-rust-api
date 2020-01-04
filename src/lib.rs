mod big_int;
pub mod kv;
pub mod native;
pub mod params;

pub use big_int::BigInt;

pub type Address = Vec<u8>; // AccAddress

#[macro_export]
macro_rules! sce_malloc {
    () => {
        #[no_mangle]
        pub extern "C" fn sce_malloc(size: i32) -> i32 {
            use std::alloc::{alloc, dealloc, Layout};
            unsafe {
                let layout = Layout::from_size_align(size as usize, 1).unwrap(); // TODO
                let ptr = alloc(layout);
                ptr as i32
            }
        }
    };
}

#[macro_export]
macro_rules! handle {
    ( $( $f:ident ( $( $a:tt ),* ) ),* ) => {
        #[no_mangle]
        pub extern "C" fn handle() {
            let route = athena::get_route();
            $(
                if route == stringify!($f) {
                    let mut idx = 0i32;
                    $f(
                        $( athena::get_param!(athena::__arg_idx(&mut idx), $a), )*
                    );
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! get_param {
    ( $i:expr, i32 ) => {
        athena::params::get_i32($i).unwrap()
    };
    ( $i:expr, i64 ) => {
        athena::params::get_i64($i).unwrap()
    };
}

pub fn __arg_idx(idx: &mut i32) -> i32 {
    *idx += 1;
    *idx - 1
}

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
