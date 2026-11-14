use super::check_err;

const SYS_READ: usize = 0;
const SYS_WRITE: usize = 1;
const SYS_CLOSE: usize = 3;
const SYS_MMAP: usize = 9;
const SYS_MUNMAP: usize = 11;
const SYS_EXIT: usize = 60;
const SYS_OPENAT: usize = 257;
const SYS_SOCKET: usize = 41;
const SYS_CONNECT: usize = 42;
const SYS_ACCEPT: usize = 43;
const SYS_BIND: usize = 49;
const SYS_LISTEN: usize = 50;
const SYS_CLOCK_GETTIME: usize = 228;
const SYS_EPOLL_WAIT: usize = 232;
const SYS_EPOLL_CTL: usize = 233;
const SYS_EPOLL_CREATE1: usize = 291;
const SYS_ACCEPT4: usize = 288;
const SYS_BPF: usize = 321;
const SYS_IO_URING_SETUP: usize = 425;
const SYS_IO_URING_ENTER: usize = 426;
const SYS_IO_URING_REGISTER: usize = 427;

#[inline(always)]
pub fn syscall1(n: usize, a1: usize) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
}

#[inline(always)]
pub fn syscall2(n: usize, a1: usize, a2: usize) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
}

#[inline(always)]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
}

#[inline(always)]
pub fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
}

#[inline(always)]
pub fn syscall5(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
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
) -> Result<usize, i32> {
    let ret: usize;
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
    check_err(ret)
}

#[inline(always)]
pub fn bpf(cmd: i32, attr: *mut u8, size: u32) -> Result<usize, i32> {
    syscall3(SYS_BPF, cmd as usize, attr as usize, size as usize)
}

#[inline(always)]
pub fn exit(status: i32) -> ! {
    let _ = syscall1(SYS_EXIT, status as usize);
    unsafe { core::hint::unreachable_unchecked() }
}

#[inline(always)]
pub fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize, i32> {
    syscall3(SYS_READ, fd as usize, buf as usize, count)
}

#[inline(always)]
pub fn write(fd: i32, buf: *const u8, count: usize) -> Result<usize, i32> {
    syscall3(SYS_WRITE, fd as usize, buf as usize, count)
}

#[inline(always)]
pub fn close(fd: i32) -> Result<usize, i32> {
    syscall1(SYS_CLOSE, fd as usize)
}

#[inline(always)]
pub fn openat(dirfd: i32, pathname: *const u8, flags: i32, mode: u32) -> Result<usize, i32> {
    syscall4(
        SYS_OPENAT,
        dirfd as usize,
        pathname as usize,
        flags as usize,
        mode as usize,
    )
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
    syscall6(
        SYS_MMAP,
        addr as usize,
        len,
        prot as usize,
        flags as usize,
        fd as usize,
        off,
    )
}

#[inline(always)]
pub fn munmap(addr: *mut u8, len: usize) -> Result<usize, i32> {
    syscall2(SYS_MUNMAP, addr as usize, len)
}

#[inline(always)]
pub fn socket(domain: i32, ty: i32, protocol: i32) -> Result<usize, i32> {
    syscall3(SYS_SOCKET, domain as usize, ty as usize, protocol as usize)
}

#[inline(always)]
pub fn bind(sockfd: i32, addr: *const u8, addrlen: u32) -> Result<usize, i32> {
    syscall3(SYS_BIND, sockfd as usize, addr as usize, addrlen as usize)
}

#[inline(always)]
pub fn listen(sockfd: i32, backlog: i32) -> Result<usize, i32> {
    syscall2(SYS_LISTEN, sockfd as usize, backlog as usize)
}

#[inline(always)]
pub fn accept(sockfd: i32, addr: *mut u8, addrlen: *mut u32) -> Result<usize, i32> {
    syscall3(SYS_ACCEPT, sockfd as usize, addr as usize, addrlen as usize)
}

#[inline(always)]
pub fn accept4(sockfd: i32, addr: *mut u8, addrlen: *mut u32, flags: i32) -> Result<usize, i32> {
    syscall4(
        SYS_ACCEPT4,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
        flags as usize,
    )
}

#[inline(always)]
pub fn clock_gettime(clk_id: i32, tp: *mut u8) -> Result<usize, i32> {
    syscall2(SYS_CLOCK_GETTIME, clk_id as usize, tp as usize)
}

#[inline(always)]
pub fn connect(sockfd: i32, addr: *const u8, addrlen: u32) -> Result<usize, i32> {
    syscall3(
        SYS_CONNECT,
        sockfd as usize,
        addr as usize,
        addrlen as usize,
    )
}

#[inline(always)]
pub fn epoll_create1(flags: i32) -> Result<usize, i32> {
    syscall1(SYS_EPOLL_CREATE1, flags as usize)
}

#[inline(always)]
pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut u8) -> Result<usize, i32> {
    syscall4(
        SYS_EPOLL_CTL,
        epfd as usize,
        op as usize,
        fd as usize,
        event as usize,
    )
}

#[inline(always)]
pub fn epoll_wait(epfd: i32, events: *mut u8, maxevents: i32, timeout: i32) -> Result<usize, i32> {
    syscall4(
        SYS_EPOLL_WAIT,
        epfd as usize,
        events as usize,
        maxevents as usize,
        timeout as usize,
    )
}

#[inline(always)]
pub fn io_uring_setup(entries: u32, p: *mut u8) -> Result<usize, i32> {
    syscall2(SYS_IO_URING_SETUP, entries as usize, p as usize)
}

#[inline(always)]
pub fn io_uring_enter(
    fd: i32,
    to_submit: u32,
    min_complete: u32,
    flags: u32,
    sig: *const u8,
) -> Result<usize, i32> {
    syscall5(
        SYS_IO_URING_ENTER,
        fd as usize,
        to_submit as usize,
        min_complete as usize,
        flags as usize,
        sig as usize,
    )
}

#[inline(always)]
pub fn io_uring_register(fd: i32, opcode: u32, arg: *const u8, nr_args: u32) -> Result<usize, i32> {
    syscall4(
        SYS_IO_URING_REGISTER,
        fd as usize,
        opcode as usize,
        arg as usize,
        nr_args as usize,
    )
}
