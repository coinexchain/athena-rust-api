fn main() {
    // println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn sce_malloc(size: i32) -> i32 {
    return 0; // TODO
}

#[no_mangle]
pub extern "C" fn handle() {
    // TODO
}
