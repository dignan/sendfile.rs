#![crate_name="sendfile"]

#[macro_use]
extern crate syscall;

#[cfg(target_os = "linux")]
#[path="linux.rs"]
pub mod platform;
