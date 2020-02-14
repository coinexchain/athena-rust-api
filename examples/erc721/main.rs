// https://github.com/AYIDouble/Simple-Game-ERC-721-Token-Template
use athena_rust_api as athena;
use athena_rust_api::{events, kv, HostStr};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

athena::init!(_init);
athena::sce_malloc!();
athena::handle!(
    balance_of(str),
    owner_of(str),
    safe_transfer_from(str, str, str),
    approve(str, str),
    get_approved(str),
    set_approval_for_all(str, str, str),
    is_approved_for_all(str, str)
);

fn balance_of(addr: &str) {
    let bs = &get_balance(addr);
    //    let bs = &get_balance_reversed(addr);
    events::emit("erc721", &[("event", "balance"), ("addr", addr), ("val", bs)]);
}

fn owner_of(token_id: &str) {
    let val = get_owner_of(token_id);
    if val.is_some() {
        events::emit(
            "'erc721",
            &[("event", "ownerOf"), ("tokenId", token_id), ("owner", val.unwrap())],
        );
    } else {
        events::emit("'erc721", &[("event", "ownerOf"), ("tokenId", token_id), ("owner", "")]);
    }
}

fn safe_transfer_from(from: &str, to: &str, token_id: &str) {
    let owner = get_owner_of(token_id);
    let sender = athena::get_caller_bech32();
    let approved_addr = get_approved_for_token(token_id);
    if !approved_addr.is_empty() {
        if !sender.eq(approved_addr) || !owner.unwrap().eq(from) {
            events::emit(
                "erc721",
                &[(
                    "error",
                    &format!(
                        "{} or {} is not equal to approved addr or owner of token respectively",
                        sender, from
                    ),
                )],
            );
        } else {
            remove_nft(token_id, from);
            remove_approval(token_id);
            add_nft(token_id, to);
            events::emit(
                "erc721",
                &[
                    ("event", "safeTransferFrom"),
                    ("tokenId", token_id),
                    ("from", from),
                    ("to", to),
                ],
            );
        }
        return;
    }
    if owner.is_some() {
        let is_approved_for_all: bool = FromStr::from_str(get_is_approved_for_all(owner.unwrap(), sender)).unwrap();
        if owner.unwrap().eq(from) && (is_approved_for_all || owner.unwrap().eq(sender)) {
            remove_nft(token_id, from);
            add_nft(token_id, to);
            events::emit(
                "erc721",
                &[
                    ("event", "safeTransferFrom"),
                    ("tokenId", token_id),
                    ("from", from),
                    ("to", to),
                ],
            );
        } else {
            events::emit(
                "erc721",
                &[
                    ("event", "safeTransferFrom"),
                    (
                        "panic",
                        &format!("{} or {} is not the owner of token {}", from, sender, token_id),
                    ),
                ],
            );
        }
        return;
    }
    events::emit(
        "erc721",
        &[
            ("event", "safeTransferFrom"),
            ("error", &format!("token not exist: {}", token_id)),
        ],
    );
}

fn _init() {
    let caller = athena::get_caller_bech32();
    add_nft("1", caller);
    add_nft("2", caller);
    add_nft("3", caller);
}

fn approve(to: &str, token_id: &str) {
    let sender = athena::get_caller_bech32();
    let owner = get_owner_of(token_id);
    if owner.is_some() {
        if !owner.unwrap().eq(sender) {
            events::emit(
                "erc721",
                &[("error", &format!("msg sender {} is not token's owner", sender))],
            );
        } else {
            let approval = get_approved_for_token(token_id);
            if !approval.is_empty() {
                events::emit(
                    "erc721",
                    &[(
                        "error",
                        &format!("token {} is already approved for {}", token_id, approval),
                    )],
                );
                return;
            }
            approve_nft(token_id, to);
            events::emit(
                "erc721",
                &[
                    ("event", "approve_token"),
                    ("tokenId", token_id),
                    ("from", sender),
                    ("to", to),
                ],
            );
        }
    } else {
        events::emit(
            "erc721",
            &[
                ("event", "approve_token"),
                ("error", &format!("token not exist: {}", token_id)),
            ],
        );
    }
}
fn get_approved(token_id: &str) {
    let approval = get_approved_for_token(token_id);
    events::emit("erc721", &[("event", "get_approved"), ("approved_addr", approval)]);
}

fn set_approval_for_all(owner: &str, operator: &str, set_or_revoke: &str) {
    let sender = athena::get_caller_bech32();
    if !sender.eq(owner) {
        return events::emit(
            "erc721",
            &[
                ("event", "set_approval_for_all"),
                ("error", &format!("sender {} is not equal to owner {}", sender, owner)),
            ],
        );
    }
    kv::set_str(&get_approved_all_key(operator, owner), &set_or_revoke);
    events::emit(
        "erc721",
        &[
            ("event", "set_approval_for_all"),
            ("owner", owner),
            ("operator", operator),
        ],
    );
}
fn is_approved_for_all(owner: &str, operator: &str) {
    let set_or_revoke = get_is_approved_for_all(owner, operator);
    events::emit(
        "erc721",
        &[("event", "is_approved_for_all"), ("is_approved_for_all", set_or_revoke)],
    );
}
fn get_balance(addr: &str) -> String {
    let mut res = String::new();
    let min = get_addr_token(addr, true);
    let max = get_addr_token(addr, false);
    let iter = kv::iterator_new(
        get_balance_key(addr, min).as_bytes(),
        get_balance_key(addr, max).as_bytes(),
    );
    loop {
        let pair = kv::iterator_next(iter);
        match pair {
            Some((_, val)) => {
                res.push_str(unsafe { std::str::from_utf8_unchecked(&val) });
                res.push_str(";");
            }

            _ => {
                kv::iterator_close(iter);
                break;
            }
        }
    }
    res
}
fn get_balance_reversed(addr: &str) -> String {
    let mut res = String::new();
    let min = get_addr_token(addr, true);
    let max = get_addr_token(addr, false);
    let iter = kv::reverse_iterator_new(
        get_balance_key(addr, min).as_bytes(),
        get_balance_key(addr, max).as_bytes(),
    );
    loop {
        let pair = kv::reverse_iterator_next(iter);
        match pair {
            Some((_, val)) => {
                res.push_str(unsafe { std::str::from_utf8_unchecked(&val) });
                res.push_str(";");
            }

            _ => {
                kv::reverse_iterator_close(iter);
                break;
            }
        }
    }
    res
}

fn get_owner_of(token_id: &str) -> Option<&str> {
    kv::get_str(&get_owner_key(token_id))
}
fn set_owner(token_id: &str, addr: &str) {
    kv::set_str(&get_owner_key(token_id), addr);
}
fn add_nft(token_id: &str, addr: &str) {
    kv::set_str(&get_balance_key(addr, token_id), token_id);
    set_owner(token_id, addr);
    _set_addr_token(token_id, addr);
}

fn _set_addr_token(token_id: &str, addr: &str) {
    let curr_min = get_addr_token(addr, true);
    let curr_max = get_addr_token(addr, false);
    if token_id.lt(curr_min) || curr_min.is_empty() {
        kv::set_str(&format!("{} min", addr), token_id);
    } else if token_id.gt(curr_max) || curr_max.is_empty() {
        kv::set_str(&format!("{} max", addr), token_id);
    }
}
fn get_addr_token(addr: &str, is_min: bool) -> HostStr {
    let curr = if is_min {
        kv::get_str(&format!("{} min", addr))
    } else {
        kv::get_str(&format!("{} max", addr))
    };
    if curr.is_some() {
        curr.unwrap()
    } else {
        &""
    }
}
fn get_addr_min(addr: &str) {
    let min = get_addr_token(addr, true);
    events::emit("erc721", &[("min", min)]);
}
fn get_addr_max(addr: &str) {
    let max = get_addr_token(addr, false);
    events::emit("erc721", &[("max", max)]);
}
fn remove_nft(token_id: &str, addr: &str) {
    kv::del_str(&format!("{}+{}", addr, token_id));
}
fn approve_nft(token_id: &str, addr: &str) {
    kv::set_str(&get_approve_for_token_key(token_id), addr);
}
fn remove_approval(token_id: &str) {
    kv::del_str(&get_approve_for_token_key(token_id));
}
fn get_approved_for_token(token_id: &str) -> &str {
    let approval = kv::get_str(&get_approve_for_token_key(token_id));
    if approval.is_some() {
        approval.unwrap()
    } else {
        &""
    }
}
fn get_is_approved_for_all(owner: &str, operator: &str) -> HostStr {
    let set_or_revoke = kv::get_str(&get_approved_all_key(operator, owner));
    if set_or_revoke.is_some() {
        set_or_revoke.unwrap()
    } else {
        &"false"
    }
}
fn get_approved_all_key(operator: &str, owner: &str) -> String {
    format!("approvedAll+{}+{}", operator, owner)
}
fn get_approve_for_token_key(token_id: &str) -> String {
    format!("approval+{}", token_id)
}
fn get_owner_key(token_id: &str) -> String {
    format!("tokenOwner+{}", token_id)
}
fn get_balance_key(addr: &str, token_id: &str) -> String {
    format!("{}+{}", addr, token_id)
}
