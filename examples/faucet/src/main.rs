use athena_rust_api as athena;
use athena_rust_api::{big, native};
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    // println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn sce_malloc(size: i32) -> i32 {
    unsafe {
        let layout = Layout::from_size_align(size as usize, 1).unwrap(); // TODO
        let ptr = alloc(layout);
        ptr as i32
    }
}

#[no_mangle]
pub extern "C" fn handle() {
    let route = athena::get_route();
    if route == "withdraw" {
        // TODO
    }
}

#[no_mangle]
pub extern "C" fn deposit(amt: i64) {
    // TODO
}

#[no_mangle]
pub extern "C" fn withdraw(amt: i64) {
    let caller = athena::get_caller();
    let amt_big = big::int_from_i64(amt);
    // TODO
}
