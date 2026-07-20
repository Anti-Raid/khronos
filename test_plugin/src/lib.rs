#[link(wasm_import_module = "env")]
extern "C" {
    fn send(ptr: *const u8, len: u32);
    fn recv_len() -> u32;
    fn recv_into(ptr: *mut u8) -> u32;
}

#[no_mangle]
pub extern "C" fn main() {
    // Send a hello message to the host
    let msg = b"Hello from WASM worker!";
    unsafe { send(msg.as_ptr(), msg.len() as u32) };

    loop {
        // Ask the host for the length of the next message
        let len = unsafe { recv_len() };
        
        // Allocate memory for it
        let mut buf = vec![0u8; len as usize];
        
        // Ask the host to write into our pointer
        unsafe { recv_into(buf.as_mut_ptr()) };
        
        // Let's do some "work" on the data (just an echo prefix)
        let prefix = b"[WASM Echo] You sent: ";
        let mut reply = Vec::with_capacity(prefix.len() + buf.len());
        reply.extend_from_slice(prefix);
        reply.extend_from_slice(&buf);
        
        // Send the result back to the host
        unsafe { send(reply.as_ptr(), reply.len() as u32) };
    }
}
