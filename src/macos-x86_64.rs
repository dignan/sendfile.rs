use std::os::unix::io::RawFd;

use std::io::Error;

pub fn sendfile(out_fd: RawFd, in_fd: RawFd, offset: i64, count: i64) -> Result<i64, Error> {
    let mut len: i64 = count.clone();
    let ret = sendfile_syscall(out_fd, in_fd, offset, &mut len, 0);

    if (ret as isize) != 0 {
        return Err(Error::from_raw_os_error(ret as i32));
    } else {
        return Ok(len);
    }
}

pub fn sendfile_all(out_fd: RawFd, in_fd: RawFd, offset: i64) -> Result<i64, Error> {
    let len = 0; // len == 0 means send until EOF
    return sendfile(out_fd, in_fd, offset, len);
}

fn sendfile_syscall(fd: RawFd, sock: RawFd, offset: i64, len: &mut i64, flags: isize) -> isize {
    unsafe {
        let mut ret: isize = 2 << 24 | ((!(0xFF << 24)) & 337);
        asm!("syscall" :
            "+{rax}"(ret) :
            "{rdi}"(fd), "{rsi}"(sock), "{rdx}"(offset), "{r10}"(len), "{r8}"(0), "{r9}"(flags) :
            "rdi", "rsi", "rdx", "r10", "r8", "r9", "memory" :
            "volatile");
        return ret
    }
}
