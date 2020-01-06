use athena_rust_api as athena;
use athena_rust_api::{native, BigInt};

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(deposit(i64), withdraw(i64));

#[no_mangle]
pub extern "C" fn deposit(amt: i64) {
    // TODO
}

#[no_mangle]
pub extern "C" fn withdraw(amt: i64) {
    let caller = athena::get_caller();
    let amt_big = BigInt::from_i64(amt);
    athena::transfer(&caller, &amt_big);
}
