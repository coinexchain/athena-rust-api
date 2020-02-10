// https://github.com/AYIDouble/Simple-Game-ERC-721-Token-Template
use athena_rust_api as athena;
use athena_rust_api::{events,kv,BigInt,HostStr,HostData};

fn main() {
    println!("Hello, world!");
}


athena::sce_malloc!();
athena::handle!(balance_of(str),owner_of(str),safe_transfer_from(str,str,str));

pub extern "C" fn balance_of(addr: &str) {
    let bs = get_balance(addr);
    events::publish("erc721", &[("event", "balance"),("addr",addr), ("val", bs)]);
}

pub extern "C" fn owner_of(token_id:HostStr){
    let val =get_owner_of(token_id);
    if val.is_some(){
        events::publish("'erc721",&[("event","ownerOf"),("tokenId",token_id),("owner",val.unwrap())]);
    }else{
        events::publish("'erc721",&[("event","ownerOf"),("tokenId",token_id),("owner","")]);
    }
}

pub extern "C" fn safe_transfer_from(from:&str, to:&str, token_id:HostStr){
    let val=get_owner_of(token_id);
    if val.is_some(){
        if !val.unwrap().eq(from){
            events::publish("erc721",&[("event","safeTransferFrom"),("panic",&format!("{} is not the owner of token {}",from,token_id))]);
        }else {
            remove_nft(token_id,from);
            add_nft(token_id,to);
            events::publish("erc721",&[("event","safeTransferFrom"),("tokenId",token_id),("from",from),("to",to)]);
        }
    }else{
        events::publish("erc721",&[("event","safeTransferFrom"),("error",&format!("token not exist: {}",token_id))]);
    }
}

pub extern "C" fn get_balance(addr: &str) -> HostStr {
    let val = kv::get_str(addr);
    if val.is_some(){
        val.unwrap()
    }else {
        &""
    }
}

pub extern "C" fn init(){
}

fn get_owner_of(token_id:HostStr)->Option<HostStr>{
    kv::get_str(&format!("tokenOwner+{}",token_id))
}
fn set_owner(token_id:HostStr,addr:&str){
    kv::set_str(&format!("tokenOwner+{}",token_id),addr)
}
fn add_nft(token_id:HostStr,addr:&str){
    let mut val = get_balance(addr);
    kv::set_str(addr,&format!("{};{}",val,token_id));
    set_owner(token_id,addr);
}
fn remove_nft(token_id:HostStr,addr:&str){
    let val = get_balance(addr);
    let mut newVal=val.replace(token_id,"");
    newVal = newVal.replace(";;",";");
    kv::set_str(addr,&newVal[..]);
}