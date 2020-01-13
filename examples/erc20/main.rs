use athena_rust_api as athena;
use athena_rust_api::{kv, BigInt};

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(transfer(bytes, i64));

// Given an address and amount, transfers that amount of tokens to that address,
// from the balance of the address that executed the transfer.
#[no_mangle]
pub extern "C" fn transfer(to_addr: &[u8], amt: i64) {
    if amt < 0 {
        return;
    }

    let amt_big = BigInt::from_i64(amt);
    let sender = athena::get_caller();
    let sender_balance = get_balance(&sender);
    if sender_balance.lt(&amt_big) {
        return;
    }

    let receiver_balance = get_balance(&to_addr);
    let sender_balance = sender_balance.sub(&amt_big);
    let receiver_balance = receiver_balance.add(&amt_big);
}

fn get_balance(addr: &[u8]) -> BigInt {
    let bytes = kv::get(addr);
    if bytes.is_some() {
        BigInt::from_bytes(&bytes.unwrap())
    } else {
        BigInt::zero()
    }
}
