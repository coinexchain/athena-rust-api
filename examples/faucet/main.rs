use athena_rust_api as athena;
use athena_rust_api::BigInt;

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::handle!(deposit(i64), withdraw(i64));

fn deposit(_amt: i64) {
    // TODO
}

fn withdraw(amt: i64) {
    let caller = athena::get_caller();
    let amt_big = BigInt::from_i64(amt);
    athena::transfer(&caller, &amt_big);
}
