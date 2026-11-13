#[inline(always)]
pub fn check_err(ret: usize) -> Result<usize, i32> {
    let ret_isize = ret as isize;
    if ret_isize > -4096 && ret_isize < 0 {
        Err(-(ret_isize as i32))
    } else {
        Ok(ret)
    }
}

#[cfg(target_arch = "x86_64")]
pub mod sys {
    pub const READ: usize = 0;
    pub const WRITE: usize = 1;
    pub const OPEN: usize = 2;
    pub const CLOSE: usize = 3;
    pub const MMAP: usize = 9;
    pub const MPROTECT: usize = 10;
    pub const MUNMAP: usize = 11;
    pub const IOCTL: usize = 16;
    pub const SCHED_YIELD: usize = 24;
    pub const MADVISE: usize = 28;
    pub const SOCKET: usize = 41;
    pub const CONNECT: usize = 42;
    pub const ACCEPT: usize = 43;
    pub const SENDTO: usize = 44;
    pub const RECVFROM: usize = 45;
    pub const SENDMSG: usize = 46;
    pub const RECVMSG: usize = 47;
    pub const BIND: usize = 49;
    pub const LISTEN: usize = 50;
    pub const SETSOCKOPT: usize = 54;
    pub const GETSOCKOPT: usize = 55;
    pub const EXIT: usize = 60;
    pub const FCNTL: usize = 72;
    pub const FUTEX: usize = 202;
    pub const MLOCK: usize = 149;
    pub const MUNLOCK: usize = 150;
    pub const MLOCKALL: usize = 151;
    pub const MUNLOCKALL: usize = 152;
    pub const SCHED_SETAFFINITY: usize = 203;
    pub const SCHED_GETAFFINITY: usize = 204;
    pub const CLOCK_GETTIME: usize = 228;
    pub const EPOLL_WAIT: usize = 232;
    pub const EPOLL_CTL: usize = 233;
    pub const MBIND: usize = 237;
    pub const SET_MEMPOLICY: usize = 238;
    pub const GET_MEMPOLICY: usize = 239;
    pub const TIMERFD_CREATE: usize = 283;
    pub const TIMERFD_SETTIME: usize = 286;
    pub const ACCEPT4: usize = 288;
    pub const EVENTFD2: usize = 290;
    pub const EPOLL_CREATE1: usize = 291;
    pub const RECVMMSG: usize = 299;
    pub const SENDMMSG: usize = 307;
    pub const BPF: usize = 321;
    pub const IO_URING_SETUP: usize = 425;
    pub const IO_URING_ENTER: usize = 426;
    pub const IO_URING_REGISTER: usize = 427;
}

#[cfg(target_arch = "aarch64")]
pub mod sys {
    pub const EVENTFD2: usize = 19;
    pub const EPOLL_CREATE1: usize = 20;
    pub const EPOLL_CTL: usize = 21;
    pub const EPOLL_WAIT: usize = 22;
    pub const FCNTL: usize = 25;
    pub const IOCTL: usize = 29;
    pub const OPENAT: usize = 56;
    pub const CLOSE: usize = 57;
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const TIMERFD_CREATE: usize = 85;
    pub const TIMERFD_SETTIME: usize = 86;
    pub const EXIT: usize = 93;
    pub const FUTEX: usize = 98;
    pub const CLOCK_GETTIME: usize = 113;
    pub const SCHED_SETAFFINITY: usize = 122;
    pub const SCHED_GETAFFINITY: usize = 123;
    pub const SCHED_YIELD: usize = 124;
    pub const SOCKET: usize = 198;
    pub const BIND: usize = 200;
    pub const LISTEN: usize = 201;
    pub const ACCEPT: usize = 202;
    pub const CONNECT: usize = 203;
    pub const SENDTO: usize = 206;
    pub const RECVFROM: usize = 207;
    pub const SETSOCKOPT: usize = 208;
    pub const GETSOCKOPT: usize = 209;
    pub const SENDMSG: usize = 211;
    pub const RECVMSG: usize = 212;
    pub const MUNMAP: usize = 215;
    pub const MMAP: usize = 222;
    pub const MPROTECT: usize = 226;
    pub const MLOCK: usize = 228;
    pub const MUNLOCK: usize = 229;
    pub const MLOCKALL: usize = 230;
    pub const MUNLOCKALL: usize = 231;
    pub const MADVISE: usize = 233;
    pub const MBIND: usize = 235;
    pub const SET_MEMPOLICY: usize = 236;
    pub const GET_MEMPOLICY: usize = 237;
    pub const ACCEPT4: usize = 242;
    pub const RECVMMSG: usize = 243;
    pub const SENDMMSG: usize = 269;
    pub const BPF: usize = 280;
    pub const IO_URING_SETUP: usize = 425;
    pub const IO_URING_ENTER: usize = 426;
    pub const IO_URING_REGISTER: usize = 427;
}

#[inline(always)]
pub fn syscall0(n: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            in("x2") a3,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8") a5,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let ret: usize;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8") a5,
            in("r9") a6,
            out("rcx") _, out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            in("x5") a6,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
pub fn exit(status: i32) -> ! {
    syscall1(sys::EXIT, status as usize);
    unsafe { core::hint::unreachable_unchecked() }
}

#[inline(always)]
pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, i32> {
    check_err(syscall3(sys::READ, fd as usize, buf as usize, count))
}

#[inline(always)]
pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<usize, i32> {
    check_err(syscall3(sys::WRITE, fd as usize, buf as usize, count))
}

#[inline(always)]
pub fn close(fd: i32) -> Result<usize, i32> {
    check_err(syscall1(sys::CLOSE, fd as usize))
}

#[inline(always)]
pub fn ioctl(fd: i32, cmd: usize, arg: usize) -> Result<usize, i32> {
    check_err(syscall3(sys::IOCTL, fd as usize, cmd, arg))
}

#[inline(always)]
pub fn fcntl(fd: i32, cmd: i32, arg: usize) -> Result<usize, i32> {
    check_err(syscall3(sys::FCNTL, fd as usize, cmd as usize, arg))
}

#[inline(always)]
pub fn mmap(
    addr: *mut u8,
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    off: usize,
) -> Result<usize, i32> {
    check_err(syscall6(
        sys::MMAP,
        addr as usize,
        len,
        prot as usize,
        flags as usize,
        fd as usize,
        off,
    ))
}

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> Result<usize, i32> {
    check_err(syscall3(sys::MPROTECT, addr as usize, len, prot as usize))
}

#[inline(always)]
pub fn munmap(addr: *mut u8, len: usize) -> Result<usize, i32> {
    check_err(syscall2(sys::MUNMAP, addr as usize, len))
}

#[inline(always)]
pub fn madvise(addr: *mut u8, len: usize, advice: i32) -> Result<usize, i32> {
    check_err(syscall3(sys::MADVISE, addr as usize, len, advice as usize))
}

#[inline(always)]
pub fn mlock(addr: *mut u8, len: usize) -> Result<usize, i32> {
    check_err(syscall2(sys::MLOCK, addr as usize, len))
}

#[inline(always)]
pub fn munlock(addr: *mut u8, len: usize) -> Result<usize, i32> {
    check_err(syscall2(sys::MUNLOCK, addr as usize, len))
}

#[inline(always)]
pub fn mlockall(flags: i32) -> Result<usize, i32> {
    check_err(syscall1(sys::MLOCKALL, flags as usize))
}

#[inline(always)]
pub fn munlockall() -> Result<usize, i32> {
    check_err(syscall0(sys::MUNLOCKALL))
}

#[inline(always)]
pub fn mbind(
    addr: *mut u8,
    len: usize,
    mode: i32,
    nodemask: *const u64,
    maxnode: usize,
    flags: u32,
) -> Result<usize, i32> {
    check_err(syscall6(
        sys::MBIND,
        addr as usize,
        len,
        mode as usize,
        nodemask as usize,
        maxnode,
        flags as usize,
    ))
}

#[inline(always)]
pub fn set_mempolicy(mode: i32, nodemask: *const u64, maxnode: usize) -> Result<usize, i32> {
    check_err(syscall3(
        sys::SET_MEMPOLICY,
        mode as usize,
        nodemask as usize,
        maxnode,
    ))
}

#[inline(always)]
pub fn sched_yield() -> Result<usize, i32> {
    check_err(syscall0(sys::SCHED_YIELD))
}

#[inline(always)]
pub fn sched_setaffinity(pid: i32, cpusetsize: usize, mask: *const u8) -> Result<usize, i32> {
    check_err(syscall3(
        sys::SCHED_SETAFFINITY,
        pid as usize,
        cpusetsize,
        mask as usize,
    ))
}

#[inline(always)]
pub fn sched_getaffinity(pid: i32, cpusetsize: usize, mask: *mut u8) -> Result<usize, i32> {
    check_err(syscall3(
        sys::SCHED_GETAFFINITY,
        pid as usize,
        cpusetsize,
        mask as usize,
    ))
}

#[inline(always)]
pub fn clock_gettime(clk_id: i32, tp: *mut u8) -> Result<usize, i32> {
    check_err(syscall2(sys::CLOCK_GETTIME, clk_id as usize, tp as usize))
}

#[inline(always)]
pub fn epoll_create1(flags: i32) -> Result<usize, i32> {
    check_err(syscall1(sys::EPOLL_CREATE1, flags as usize))
}

#[inline(always)]
pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut u8) -> Result<usize, i32> {
    check_err(syscall4(
        sys::EPOLL_CTL,
        epfd as usize,
        op as usize,
        fd as usize,
        event as usize,
    ))
}

#[inline(always)]
pub fn epoll_wait(epfd: i32, events: *mut u8, maxevents: i32, timeout: i32) -> Result<usize, i32> {
    check_err(syscall4(
        sys::EPOLL_WAIT,
        epfd as usize,
        events as usize,
        maxevents as usize,
        timeout as usize,
    ))
}

#[inline(always)]
pub fn eventfd2(initval: u32, flags: i32) -> Result<usize, i32> {
    check_err(syscall2(sys::EVENTFD2, initval as usize, flags as usize))
}

#[inline(always)]
pub fn timerfd_create(clockid: i32, flags: i32) -> Result<usize, i32> {
    check_err(syscall2(
        sys::TIMERFD_CREATE,
        clockid as usize,
        flags as usize,
    ))
}

#[inline(always)]
pub fn timerfd_settime(
    fd: i32,
    flags: i32,
    new_value: *const u8,
    old_value: *mut u8,
) -> Result<usize, i32> {
    check_err(syscall4(
        sys::TIMERFD_SETTIME,
        fd as usize,
        flags as usize,
        new_value as usize,
        old_value as usize,
    ))
}

#[inline(always)]
pub fn futex(
    uaddr: *mut u32,
    op: i32,
    val: u32,
    utime: *const u8,
    uaddr2: *mut u32,
    val3: u32,
) -> Result<usize, i32> {
    check_err(syscall6(
        sys::FUTEX,
        uaddr as usize,
        op as usize,
        val as usize,
        utime as usize,
        uaddr2 as usize,
        val3 as usize,
    ))
}

#[inline(always)]
pub fn bpf(cmd: i32, attr: *mut u8, size: u32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::BPF,
        cmd as usize,
        attr as usize,
        size as usize,
    ))
}

#[inline(always)]
pub fn socket(domain: i32, ty: i32, protocol: i32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::SOCKET,
        domain as usize,
        ty as usize,
        protocol as usize,
    ))
}

#[inline(always)]
pub fn bind(sockfd: i32, addr: *const u8, addrlen: u32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::BIND,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
    ))
}

#[inline(always)]
pub fn listen(sockfd: i32, backlog: i32) -> Result<usize, i32> {
    check_err(syscall2(sys::LISTEN, sockfd as usize, backlog as usize))
}

#[inline(always)]
pub fn accept(sockfd: i32, addr: *mut u8, addrlen: *mut u32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::ACCEPT,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
    ))
}

#[inline(always)]
pub fn accept4(sockfd: i32, addr: *mut u8, addrlen: *mut u32, flags: i32) -> Result<usize, i32> {
    check_err(syscall4(
        sys::ACCEPT4,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
        flags as usize,
    ))
}

#[inline(always)]
pub fn connect(sockfd: i32, addr: *const u8, addrlen: u32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::CONNECT,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
    ))
}

#[inline(always)]
pub fn sendto(
    sockfd: i32,
    buf: *const u8,
    len: usize,
    flags: i32,
    dest_addr: *const u8,
    addrlen: u32,
) -> Result<usize, i32> {
    check_err(syscall6(
        sys::SENDTO,
        sockfd as usize,
        buf as usize,
        len,
        flags as usize,
        dest_addr as usize,
        addrlen as usize,
    ))
}

#[inline(always)]
pub fn recvfrom(
    sockfd: i32,
    buf: *mut u8,
    len: usize,
    flags: i32,
    src_addr: *mut u8,
    addrlen: *mut u32,
) -> Result<usize, i32> {
    check_err(syscall6(
        sys::RECVFROM,
        sockfd as usize,
        buf as usize,
        len,
        flags as usize,
        src_addr as usize,
        addrlen as usize,
    ))
}

#[inline(always)]
pub fn sendmsg(sockfd: i32, msg: *const u8, flags: i32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::SENDMSG,
        sockfd as usize,
        msg as usize,
        flags as usize,
    ))
}

#[inline(always)]
pub fn recvmsg(sockfd: i32, msg: *mut u8, flags: i32) -> Result<usize, i32> {
    check_err(syscall3(
        sys::RECVMSG,
        sockfd as usize,
        msg as usize,
        flags as usize,
    ))
}

#[inline(always)]
pub fn sendmmsg(sockfd: i32, msgvec: *mut u8, vlen: u32, flags: i32) -> Result<usize, i32> {
    check_err(syscall4(
        sys::SENDMMSG,
        sockfd as usize,
        msgvec as usize,
        vlen as usize,
        flags as usize,
    ))
}

#[inline(always)]
pub fn recvmmsg(
    sockfd: i32,
    msgvec: *mut u8,
    vlen: u32,
    flags: i32,
    timeout: *mut u8,
) -> Result<usize, i32> {
    check_err(syscall5(
        sys::RECVMMSG,
        sockfd as usize,
        msgvec as usize,
        vlen as usize,
        flags as usize,
        timeout as usize,
    ))
}

#[inline(always)]
pub fn setsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: *const u8,
    optlen: u32,
) -> Result<usize, i32> {
    check_err(syscall5(
        sys::SETSOCKOPT,
        sockfd as usize,
        level as usize,
        optname as usize,
        optval as usize,
        optlen as usize,
    ))
}

#[inline(always)]
pub fn getsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: *mut u8,
    optlen: *mut u32,
) -> Result<usize, i32> {
    check_err(syscall5(
        sys::GETSOCKOPT,
        sockfd as usize,
        level as usize,
        optname as usize,
        optval as usize,
        optlen as usize,
    ))
}

#[inline(always)]
pub fn io_uring_setup(entries: u32, p: *mut u8) -> Result<usize, i32> {
    check_err(syscall2(sys::IO_URING_SETUP, entries as usize, p as usize))
}

#[inline(always)]
pub fn io_uring_enter(
    fd: i32,
    to_submit: u32,
    min_complete: u32,
    flags: u32,
    sig: *const u8,
) -> Result<usize, i32> {
    check_err(syscall5(
        sys::IO_URING_ENTER,
        fd as usize,
        to_submit as usize,
        min_complete as usize,
        flags as usize,
        sig as usize,
    ))
}

#[inline(always)]
pub fn io_uring_register(fd: i32, opcode: u32, arg: *const u8, nr_args: u32) -> Result<usize, i32> {
    check_err(syscall4(
        sys::IO_URING_REGISTER,
        fd as usize,
        opcode as usize,
        arg as usize,
        nr_args as usize,
    ))
}
