// MultiSig Wallet Example
// https://medium.com/@ChrisLundkvist/exploring-simpler-ethereum-multisig-contracts-b71020c19037
// https://github.com/christianlundkvist/simple-multisig/blob/master/contracts/SimpleMultiSig.sol

use athena_rust_api as athena;
use athena_rust_api::{crypto, events, kv, BigInt};

fn main() {
    println!("Hello, world!");
}

const NONCE_KEY: &'static str = "_nonce";
const THRESHOLD_KEY: &'static str = "_threshold";

athena::sce_malloc!();

athena::handle!(execute(str, str, i64, str));

#[no_mangle]
pub extern "C" fn execute(sig_data: &str, to_addr: &str, amt: i64, data: &str) {
    let nonce = BigInt::from_str(kv::get_str(NONCE_KEY).unwrap());
    let threshold = kv::get(THRESHOLD_KEY.as_bytes()).unwrap()[0];
    let amt_big = BigInt::from_i64(amt);

    // hash_data := to_addr(bech32) | amt(str) | data(hex) | nonce(str)
    let mut hash_data = Vec::new();
    hash_data.copy_from_slice(to_addr.as_bytes());
    hash_data.copy_from_slice(amt_big.to_str().as_bytes());
    hash_data.copy_from_slice(data.as_bytes());
    hash_data.copy_from_slice(amt_big.to_str().as_bytes());
    let hash = crypto::sha256(&hash_data);

    let mut n = 0;
    for addr_sig in sig_data.split(";") {
        let mut splitted = addr_sig.split(",");
        let addr_bech32 = splitted.next().unwrap();
        let sig_hex = splitted.next().unwrap();

        let addr = athena::addr_from_bech32(addr_bech32).unwrap();
        let sig = decode_hex(sig_hex);

        if kv::get_str(addr_bech32).is_some() {
            if crypto::verify_sig(addr, hash, &sig) {
                n += 1;
            }
        }
    }

    if n >= threshold {
        let nonce = nonce.add(&BigInt::one());
        kv::set_str(NONCE_KEY, nonce.to_str());

        let to_addr_bech32 = athena::addr_from_bech32(to_addr).unwrap();
        athena::transfer(to_addr_bech32, &amt_big);
    }
}

// https://stackoverflow.com/questions/52987181/how-can-i-convert-a-hex-string-to-a-u8-slice
pub fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
