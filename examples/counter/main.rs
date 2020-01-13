use athena_rust_api as athena;
use athena_rust_api::{events, kv, BigInt};

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(incr(i64), query());

#[no_mangle]
pub extern "C" fn incr(n: i64) {
    let c = get_count();
    let x = BigInt::from_i64(n);
    let new_c = c.add(&x);
    set_count(&new_c);
}

#[no_mangle]
pub extern "C" fn query() {
    let c = get_count().to_str();
    events::publish("counter", &[("count", c)]);
}

fn get_count() -> BigInt {
    let c = kv::get("__c".as_bytes());
    if c.is_none() {
        BigInt::zero()
    } else {
        BigInt::from_bytes(&c.unwrap())
    }
}

fn set_count(c: &BigInt) {
    let s = c.to_str();
    kv::set("__c".as_bytes(), s.as_bytes());
}
