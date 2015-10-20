use std::io::{Error, Result};
use std::os::unix::io::RawFd;

pub fn sendfile(out_fd: RawFd, in_fd: RawFd, offset: i64, count: i64) -> Result<i64> {
    let mut inner_offset: isize = count.clone() as isize;
    let ret = sendfile_syscall(out_fd, in_fd, &mut inner_offset, count as isize);

    if (ret as isize) != 0 {
        return Err(Error::from_raw_os_error(ret as i32));
    } else {
        return Ok(ret as i64);
    }
}


pub fn sendfile_all(out_fd: RawFd, in_fd: RawFd, chunk_size: i64) -> Result<i64> {
     let mut written = 0usize;

     loop {
        let ret = try!(sendfile(out_fd, in_fd, 0, chunk_size));

         if ret == 0 {
             // we're done
             return Ok(written as i64);
         } else {
             written += ret as usize;
         }
     }
}

fn sendfile_syscall(out_fd: RawFd, in_fd: RawFd, offset: &mut isize, count: isize) -> isize {
    unsafe {
        let mut ret = 40;
        asm!("syscall" :
            "+{rax}"(ret) :
            "{rdi}"(out_fd), "{rsi}"(in_fd), "{rdx}"(offset), "{r10}"(count) :
            "rdi", "rsi", "rdx", "r10", "memory" :
            "volatile");
        return ret
    }
}
