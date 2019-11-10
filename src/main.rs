#![allow(unused_variables)]

extern crate libc;
use libc::{c_void, write};

#[cfg(not(target_os = "windows"))]
fn syscall_libc(message: String) {
    let msg_ptr = message.as_ptr() as *const c_void;
    let len = message.len();
    unsafe { write(1, msg_ptr, len) };
}

fn main() {
    syscall_libc(String::from("poe-n\n"));
}
