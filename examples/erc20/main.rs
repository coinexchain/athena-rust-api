use athena_rust_api as athena;
use athena_rust_api::{events, kv, BigInt};

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(total_supply(), balance_of(addr), transfer(addr, i64));

pub extern "C" fn total_supply() {
    let ts = kv::get_str("__total_supply").unwrap();
    events::publish("erc20", &[("event", "total_supply"), ("val", ts)]);
}

pub extern "C" fn balance_of(addr: &[u8]) {
    let bs = get_balance(addr).to_str();
    events::publish("erc20", &[("event", "balance"), ("val", bs)]);
}

// Given an address and amount, transfers that amount of tokens to that address,
// from the balance of the address that executed the transfer.
#[no_mangle]
pub extern "C" fn transfer(to_addr: &[u8], amt: i64) {
    if amt < 0 {
        return;
    }

    let amt_big = BigInt::from_i64(amt);
    let sender_addr = athena::get_caller();
    let sender_balance = get_balance(sender_addr);
    if sender_balance.lt(&amt_big) {
        return;
    }

    let receiver_balance = get_balance(to_addr);
    let sender_balance = sender_balance.sub(&amt_big);
    let receiver_balance = receiver_balance.add(&amt_big);

    update_balance(sender_addr, &sender_balance);
    update_balance(to_addr, &receiver_balance);
    events::publish(
        "erc20",
        &[
            ("event", "transfer"),
            // ("to", to_addr), // TODO
            ("amt", amt_big.to_str()),
        ],
    );
}

fn get_balance(addr: &[u8]) -> BigInt {
    let bytes = kv::get(addr);
    if bytes.is_some() {
        BigInt::from_bytes(bytes.unwrap())
    } else {
        BigInt::zero()
    }
}

fn update_balance(addr: &[u8], new_val: &BigInt) {}
