use std::io::{Read, Write, Error, Result};
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;

use std::io::Error;

pub fn sendfile(out_fd: RawFd, in_fd: RawFd, offset: i64, count: i64) -> Result<i64, Error> {
    let mut len: isize = count.clone();
    let ret = sendfile_syscall(out_fd, in_fd, &mut len, count);

    if (ret as isize) != 0 {
        return Err(Error::from_raw_os_error(ret as i32));
    } else {
        return Ok(len);
    }
}


pub fn sendfile_all(out_fd: RawFd, in_fd: RawFd, offset: i64) -> Result<i64, Error> {
     let mut written = 0usize;

     loop {
        let ret = try!(sendfile(source.as_raw_fd(), sink.as_raw_fd(), chunk_size));

         if ret == 0usize {
             // we're done
             return Ok(written);
         } else {
             written += ret;
         }
     }
}

fn sendfile_syscall(out_fd: RawFd, in_fd: RawFd, offset: &mut isize, count: isize) -> isize {
    unsafe {
        asm!("syscall" :
            "+{rax}"(ret) :
            "{rdi}"(fd), "{rsi}"(sock), "{rdx}"(offset), "{r10}"(len) :
            "rdi", "rsi", "rdx", "r10", "memory" :
            "volatile");
        return ret
    }
}
