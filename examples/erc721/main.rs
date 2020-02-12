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
    let bs = get_balance(addr);
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
fn get_balance(addr: &str) -> HostStr {
    let val = kv::get_str(addr);
    if val.is_some() {
        val.unwrap()
    } else {
        &""
    }
}

fn get_owner_of(token_id: &str) -> Option<&str> {
    kv::get_str(&get_owner_key(token_id))
}
fn set_owner(token_id: &str, addr: &str) {
    kv::set_str(&get_owner_key(token_id), addr);
}
fn add_nft(token_id: &str, addr: &str) {
    let val = get_balance(addr);
    kv::set_str(addr, &format!("{};{}", val, token_id));
    set_owner(token_id, addr);
}
fn remove_nft(token_id: &str, addr: &str) {
    let val = get_balance(addr);
    if !val.contains(token_id) {
        return;
    }
    let mut new_val = val.replace(token_id, "");
    new_val = new_val.replace(";;", ";");
    kv::set_str(addr, &new_val[..]);
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
