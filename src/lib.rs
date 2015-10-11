#![crate_name="sendfile"]
#![feature(asm)]

use std::io::{Read, Write, Result};
use std::os::unix::io::AsRawFd;

/// Sends `count` bytes directly from `source` to `sink`, bypassing userspace
/// When successful, the call returns a `Result` with the number of bytes sent,
/// When the call fails, returns a `Result` with the system error
pub fn sendfile<R, W>(source: &R, sink: &W, offset: i64, count: i64) -> Result<i64>
        where R: Read + AsRawFd,
              W: Write + AsRawFd {
    return Ok(try!(self::platform::sendfile(source.as_raw_fd(), sink.as_raw_fd(), offset, count)));
}

/// Sends a file directly from `source` to `sink` in chunks of `chunk_size`, bypassing userspace
/// The chunk size may be ignored on certain platforms where implementations allow sending until EOF
/// Implementations supporting send-to-EOF include Mac OS and FreeBSD
pub fn sendfile_all<R, W>(source: &R, sink: &W, chunk_size: i64) -> Result<i64>
         where R: Read + AsRawFd,
               W: Write + AsRawFd {
     return Ok(try!(self::platform::sendfile_all(source.as_raw_fd(), sink.as_raw_fd(), chunk_size)));
 }

#[cfg(all(target_os = "linux", target_arch="x86_64"))]
#[path="linux-x86_64.rs"]
pub mod platform;

#[cfg(all(target_os = "macos", target_arch="x86_64"))]
#[path="macos-x86_64.rs"]
pub mod platform;
