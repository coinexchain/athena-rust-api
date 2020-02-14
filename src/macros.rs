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
macro_rules! init {
    ($f:ident) => {
        #[no_mangle]
        pub extern "C" fn init() -> i32 {
            $f();
            1
        }
    };
}

#[macro_export]
macro_rules! handle {
    ( $( $f:ident ( $( $a:tt ),* ) ),* ) => {
        #[no_mangle]
        pub extern "C" fn handle() -> i32 {
            let route = athena::get_route();
            athena::params::decode_as_cbor();
            $(
                if route == stringify!($f) {
                    let mut idx = 0i32;
                    $f(
                        $( athena::get_param!(athena::macros::__arg_idx(&mut idx), $a), )*
                    );
                }
            )*
            1
        }
    }
}

#[macro_export]
macro_rules! get_param {
    ( $i:expr, i32 ) => {
        athena::params::get_i32($i)
    };
    ( $i:expr, i64 ) => {
        athena::params::get_i64($i)
    };
    ( $i:expr, bytes ) => {
        athena::params::get_bytes($i)
    };
    ( $i:expr, str ) => {
        athena::params::get_str($i)
    };
    ( $i:expr, bech32 ) => {
        athena::params::get_str($i)
    };
}

pub fn __arg_idx(idx: &mut i32) -> i32 {
    *idx += 1;
    *idx - 1
}

// let x = athena::cbor_encode!(1i32, 2i64, "a");
#[macro_export]
macro_rules! cbor_encode {
    ( $($e:expr),* ) => {
        {
            athena::params::cbor_encode_begin();
            $(
                athena::params::Param::cbor_encode($e);
            )*
            athena::params::cbor_encode_end()
        }
    };
}
