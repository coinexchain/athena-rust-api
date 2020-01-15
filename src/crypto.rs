use super::{native, HostData};

pub fn verify_sig(addr: &[u8], digest: &[u8], sig: &[u8]) -> bool {
    unsafe {
        let ok = native::sci_verifysig(
            addr.as_ptr(),
            digest.as_ptr(),
            digest.len() as i32,
            sig.as_ptr(),
            sig.len() as i32,
        );
        native::is_ok(ok)
    }
}

pub fn sha256(data: &[u8]) -> HostData {
    unsafe {
        let ptr = native::sci_sha256(data.as_ptr(), data.len() as i32);
        std::slice::from_raw_parts(ptr, 32)
    }
}
