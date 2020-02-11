use athena_rust_api as athena;
use athena_rust_api::{events, kv, BigInt, HostStr};
use std::intrinsics::transmute;
use std::thread::AccessError;
use std::panic::resume_unwind;
use athena_rust_api::block::get_timestamp;

fn main() {
    // println!("Hello, world!");
}

athena::sce_malloc!();

athena::init!(_init);
athena::handle!(set_guardian(bech32), set_successor(bech32), get_guardian(), get_successor(),
                switch(), freeze(), unfreeze(), limit(str, i64));

fn _init() {
    let owner = athena::get_caller_bech32();
    kv::set_str(MASTER, owner)
}

//todo: enum?
const MASTER: &str = "master";
const GUARDIAN: &str = "guardian";
const SUCCESSOR: &str = "successor";

//**** secure module
//**** manage Guardian, Successor
fn set_guardian(addr: &str) {
    _set_address(GUARDIAN, addr)
}

fn set_successor(addr: &str) {
    _set_address(SUCCESSOR, addr)
}

fn get_guardian() {
    _get_address(GUARDIAN)
}

fn get_successor() {
    _get_address(SUCCESSOR)
}

fn _get_address(kind: &str) {
    if !_check_address_kind(kind) {
        return;
    }
    let addr = kv::get_str(kind);
    if addr.is_some() {
        return events::publish("smart_wallet", &[("event", kind), ("address", addr.unwrap())]);
    }
    return events::publish("smart_wallet", &[("event", kind), ("address", "not exist")]);
}

fn _set_address(kind: &str, address: &str) {
    //todo: identity check
    if !_check_address_kind(kind) {
        return;
    }
    kv::set_str(kind, address);
    events::publish("smart_wallet", &[("event", "set_address"), ("type", kind), ("address", address)])
}

fn _check_address_kind(kind: &str) -> bool {
    return match kind {
        SUCCESSOR | GUARDIAN => true,
        _ => false
    };
}

//**** account manager
//**** freeze, unfreeze, switch, limit
fn switch() {
    let master = kv::get_str(MASTER).unwrap();
    let caller = athena::get_caller_bech32();
    let successor = kv::get_str(SUCCESSOR);
    if successor.is_some() {
        let s = successor.unwrap();
        if master == caller || s == caller {
            kv::set_str(SUCCESSOR, master);
            kv::set_str(SUCCESSOR, s);
        } else {
            return events::publish("smart_wallet", &[("event", "switch"), ("result", "invalid caller")]);
        }
    } else {
        return events::publish("smart_wallet", &[("event", "switch"), ("result", "successor not exist")]);
    }
    return events::publish("smart_wallet", &[("event", "switch"), ("result", "successor now be master")]);
}

fn freeze() {
    let caller = athena::get_caller_bech32();
    let successor = kv::get_str(SUCCESSOR);
    if successor.is_some() {
        if successor.unwrap() == caller {
            events::publish("smart_wallet", &[("event", "freeze"), ("type", "successor freeze")]);
            return kv::set_str("successor_freeze", BigInt::from_i64(get_timestamp().0).to_str());
        }
    }
    let master = kv::get_str(MASTER).unwrap();
    if master == caller {
        events::publish("smart_wallet", &[("event", "freeze"), ("type", "master freeze")]);
        return kv::set_str("freeze", "true");
    }
    return events::publish("smart_wallet", &[("event", "freeze"), ("type", "invalid")]);
}

fn unfreeze() {
    let caller = athena::get_caller_bech32();
    let successor = kv::get_str(SUCCESSOR);
    if successor.is_some() {
        if successor.unwrap() == caller {
            events::publish("smart_wallet", &[("event", "unfreeze"), ("type", "successor unfreeze")]);
            return kv::del_str("successor_freeze");
        }
    }
    let master = kv::get_str(MASTER).unwrap();
    if master == caller {
        events::publish("smart_wallet", &[("event", "unfreeze"), ("type", "master unfreeze")]);
        kv::del_str("successor_freeze");
        return kv::del_str("freeze");
    }
    return events::publish("smart_wallet", &[("event", "unfreeze"), ("type", "invalid")]);
}

fn limit(token: &str, limit: i64) {
    let caller = athena::get_caller_bech32();
    let master = kv::get_str(MASTER).unwrap();
    if caller != master {
        let successor = kv::get_str(SUCCESSOR);
        if successor.is_some() {
            if successor.unwrap() != caller {
                return events::publish("smart_wallet", &[("event", "limit"), ("result", "invalid caller")]);
            }
        } else {
            return events::publish("smart_wallet", &[("event", "limit"), ("result", "invalid caller and successor not exist")]);
        }
    }
    let mut s: String = String::new();
    _token_limit_key(token, &mut s);
    let l = BigInt::from_i64(limit).to_str();
    kv::set_str(s.as_str(), l);
    return events::publish("smart_wallet", &[("event", "limit"), ("result", "success"), ("token", token), ("limit", l)]);
}

fn _token_key(token: &str, result: &mut String) {
    *result = "token->".to_owned() + token;
}

fn _token_limit_key(token: &str, result: &mut String) {
    *result = "token_limit->".to_owned() + token;
}

//**** transfer module
//**** token in & out

fn transfer(token: &str, to_address: &str, amount: i64) {}

fn receive(token: &str, src_address: &str, amount: i64) {}