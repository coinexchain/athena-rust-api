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
            if !athena::params::decode_as_cbor() {
                panic!();
            }
            $(
                if route == stringify!($f) {
                    let mut idx = 0i32;
                    $f(
                        $( athena::get_param!(athena::macros::__arg_idx(&mut idx), $a), )*
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
