use athena_rust_api as athena;
use athena_rust_api::{kv, BigInt};

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(transfer(i64));

// Given an address and amount, transfers that amount of tokens to that address,
// from the balance of the address that executed the transfer.
#[no_mangle]
pub extern "C" fn transfer(amt: i64) {
    if amt < 0 {
        return;
    }

    let amt_big = BigInt::from_i64(amt);
    let caller = athena::get_caller();
    let balance_bytes = kv::get(caller).unwrap();
    let balance = BigInt::from_bytes(&balance_bytes).unwrap();
    if balance.lt(&amt_big) {
        return;
    }

    // TODO
}
