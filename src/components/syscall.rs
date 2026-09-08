// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(transparent)]
pub struct Syscall(pub u64);

#[inline(always)]
pub fn check_err(ret: isize) -> Result<usize, i32> {
    if (ret as usize) > -4096isize as usize {
        Err(-(ret as i32))
    } else {
        Ok(ret as usize)
    }
}

macro_rules! syscall {
    ($name:ident, $id:ident) => {
        #[inline(always)]
        pub fn $name() -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty, $a2:ident: $t2:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1, $a2: $t2) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    in("rsi") $a2 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty, $a2:ident: $t2:ty, $a3:ident: $t3:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1, $a2: $t2, $a3: $t3) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    in("rsi") $a2 as u64,
                    in("rdx") $a3 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty, $a2:ident: $t2:ty, $a3:ident: $t3:ty, $a4:ident: $t4:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1, $a2: $t2, $a3: $t3, $a4: $t4) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    in("rsi") $a2 as u64,
                    in("rdx") $a3 as u64,
                    in("r10") $a4 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty, $a2:ident: $t2:ty, $a3:ident: $t3:ty, $a4:ident: $t4:ty, $a5:ident: $t5:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1, $a2: $t2, $a3: $t3, $a4: $t4, $a5: $t5) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    in("rsi") $a2 as u64,
                    in("rdx") $a3 as u64,
                    in("r10") $a4 as u64,
                    in("r8") $a5 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
    ($name:ident, $id:ident, $a1:ident: $t1:ty, $a2:ident: $t2:ty, $a3:ident: $t3:ty, $a4:ident: $t4:ty, $a5:ident: $t5:ty, $a6:ident: $t6:ty) => {
        #[inline(always)]
        pub fn $name($a1: $t1, $a2: $t2, $a3: $t3, $a4: $t4, $a5: $t5, $a6: $t6) -> Result<usize, i32> {
            let ret: isize;
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") Self::$id.0,
                    in("rdi") $a1 as u64,
                    in("rsi") $a2 as u64,
                    in("rdx") $a3 as u64,
                    in("r10") $a4 as u64,
                    in("r8") $a5 as u64,
                    in("r9") $a6 as u64,
                    lateout("rax") ret,
                    out("rcx") _,
                    out("r11") _,
                    options(nostack)
                );
            }
            check_err(ret)
        }
    };
}

impl Syscall {
    pub const READ: Self = Self(0);
    pub const WRITE: Self = Self(1);
    pub const OPEN: Self = Self(2);
    pub const CLOSE: Self = Self(3);
    pub const STAT: Self = Self(4);
    pub const FSTAT: Self = Self(5);
    pub const LSEEK: Self = Self(8);
    pub const MMAP: Self = Self(9);
    pub const MPROTECT: Self = Self(10);
    pub const MUNMAP: Self = Self(11);
    pub const IOCTL: Self = Self(16);
    pub const READV: Self = Self(19);
    pub const WRITEV: Self = Self(20);
    pub const SCHED_YIELD: Self = Self(24);
    pub const MREMAP: Self = Self(25);
    pub const MADVISE: Self = Self(28);
    pub const NANOSLEEP: Self = Self(35);
    pub const SOCKET: Self = Self(41);
    pub const CONNECT: Self = Self(42);
    pub const SENDTO: Self = Self(44);
    pub const RECVFROM: Self = Self(45);
    pub const SHUTDOWN: Self = Self(48);
    pub const BIND: Self = Self(49);
    pub const LISTEN: Self = Self(50);
    pub const SETSOCKOPT: Self = Self(54);
    pub const GETSOCKOPT: Self = Self(55);
    pub const CLONE: Self = Self(56);
    pub const EXECVE: Self = Self(59);
    pub const EXIT: Self = Self(60);
    pub const WAIT4: Self = Self(61);
    pub const KILL: Self = Self(62);
    pub const FCNTL: Self = Self(72);
    pub const FDATASYNC: Self = Self(75);
    pub const GETCWD: Self = Self(79);
    pub const CHDIR: Self = Self(80);
    pub const SCHED_SETSCHEDULER: Self = Self(144);
    pub const MLOCK: Self = Self(149);
    pub const MLOCKALL: Self = Self(151);
    pub const PRCTL: Self = Self(157);
    pub const GETTID: Self = Self(186);
    pub const FUTEX: Self = Self(202);
    pub const SCHED_SETAFFINITY: Self = Self(203);
    pub const GETDENTS64: Self = Self(217);
    pub const CLOCK_GETTIME: Self = Self(228);
    pub const CLOCK_NANOSLEEP: Self = Self(230);
    pub const EXIT_GROUP: Self = Self(231);
    pub const EPOLL_WAIT: Self = Self(232);
    pub const EPOLL_CTL: Self = Self(233);
    pub const OPENAT: Self = Self(257);
    pub const UNLINKAT: Self = Self(263);
    pub const SPLICE: Self = Self(275);
    pub const TEE: Self = Self(276);
    pub const TIMERFD_CREATE: Self = Self(283);
    pub const FALLOCATE: Self = Self(285);
    pub const TIMERFD_SETTIME: Self = Self(286);
    pub const ACCEPT4: Self = Self(288);
    pub const EVENTFD2: Self = Self(290);
    pub const EPOLL_CREATE1: Self = Self(291);
    pub const PIPE2: Self = Self(293);
    pub const RECVMMSG: Self = Self(299);
    pub const SENDMMSG: Self = Self(307);
    pub const GETCPU: Self = Self(309);
    pub const SECCOMP: Self = Self(317);
    pub const GETRANDOM: Self = Self(318);
    pub const MEMFD_CREATE: Self = Self(319);
    pub const BPF: Self = Self(321);
    pub const USERFAULTFD: Self = Self(323);
    pub const COPY_FILE_RANGE: Self = Self(326);
    pub const PWRITEV2: Self = Self(327);
    pub const STATX: Self = Self(332);
    pub const IO_URING_SETUP: Self = Self(425);
    pub const IO_URING_ENTER: Self = Self(426);
    pub const IO_URING_REGISTER: Self = Self(427);
    pub const PIDFD_OPEN: Self = Self(434);
    pub const CLONE3: Self = Self(435);
    pub const FUTEX_WAITV: Self = Self(448);

    #[inline(always)]
    pub fn exit_group(status: i32) -> ! {
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") Self::EXIT_GROUP.0,
                in("rdi") status as u64,
                options(noreturn, nostack)
            );
        }
    }

    syscall!(read, READ, fd: i32, buf: *mut u8, count: usize);
    syscall!(write, WRITE, fd: i32, buf: *const u8, count: usize);
    syscall!(open, OPEN, filename: *const u8, flags: i32, mode: u32);
    syscall!(close, CLOSE, fd: i32);
    syscall!(stat, STAT, filename: *const u8, statbuf: *mut u8);
    syscall!(fstat, FSTAT, fd: i32, statbuf: *mut u8);
    syscall!(lseek, LSEEK, fd: i32, offset: i64, whence: i32);
    syscall!(mmap, MMAP, addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: usize);
    syscall!(mprotect, MPROTECT, addr: *mut u8, len: usize, prot: i32);
    syscall!(munmap, MUNMAP, addr: *mut u8, len: usize);
    syscall!(ioctl, IOCTL, fd: i32, cmd: u64, arg: usize);
    syscall!(readv, READV, fd: i32, iov: *const u8, iovcnt: i32);
    syscall!(writev, WRITEV, fd: i32, iov: *const u8, iovcnt: i32);
    syscall!(sched_yield, SCHED_YIELD);
    syscall!(mremap, MREMAP, old_addr: *mut u8, old_size: usize, new_size: usize, flags: i32, new_addr: *mut u8);
    syscall!(madvise, MADVISE, addr: *mut u8, len: usize, advice: i32);
    syscall!(nanosleep, NANOSLEEP, req: *const u8, rem: *mut u8);
    syscall!(socket, SOCKET, domain: i32, socket_type: i32, protocol: i32);
    syscall!(connect, CONNECT, sockfd: i32, addr: *const u8, addrlen: u32);
    syscall!(sendto, SENDTO, sockfd: i32, buf: *const u8, len: usize, flags: i32, dest_addr: *const u8, addrlen: u32);
    syscall!(recvfrom, RECVFROM, sockfd: i32, buf: *mut u8, len: usize, flags: i32, src_addr: *mut u8, addrlen: *mut u32);
    syscall!(shutdown, SHUTDOWN, sockfd: i32, how: i32);
    syscall!(bind, BIND, sockfd: i32, addr: *const u8, addrlen: u32);
    syscall!(listen, LISTEN, sockfd: i32, backlog: i32);
    syscall!(setsockopt, SETSOCKOPT, sockfd: i32, level: i32, optname: i32, optval: *const u8, optlen: u32);
    syscall!(getsockopt, GETSOCKOPT, sockfd: i32, level: i32, optname: i32, optval: *mut u8, optlen: *mut u32);
    syscall!(clone, CLONE, flags: usize, stack: *mut u8, parent_tid: *mut i32, child_tid: *mut i32, tls: usize);
    syscall!(execve, EXECVE, filename: *const u8, argv: *const *const u8, envp: *const *const u8);
    syscall!(exit, EXIT, status: i32);
    syscall!(wait4, WAIT4, pid: i32, wstatus: *mut i32, options: i32, rusage: *mut u8);
    syscall!(kill, KILL, pid: i32, sig: i32);
    syscall!(fcntl, FCNTL, fd: i32, cmd: i32, arg: usize);
    syscall!(fdatasync, FDATASYNC, fd: i32);
    syscall!(getcwd, GETCWD, buf: *mut u8, size: usize);
    syscall!(chdir, CHDIR, filename: *const u8);
    syscall!(sched_setscheduler, SCHED_SETSCHEDULER, pid: i32, policy: i32, param: *const u8);
    syscall!(mlock, MLOCK, addr: *const u8, len: usize);
    syscall!(mlockall, MLOCKALL, flags: i32);
    syscall!(prctl, PRCTL, option: i32, arg2: usize, arg3: usize, arg4: usize, arg5: usize);
    syscall!(gettid, GETTID);
    syscall!(futex, FUTEX, uaddr: *mut u32, futex_op: i32, val: u32, timeout: *const u8, uaddr2: *mut u32, val3: u32);
    syscall!(sched_setaffinity, SCHED_SETAFFINITY, pid: usize, cpusetsize: usize, mask: *const usize);
    syscall!(getdents64, GETDENTS64, fd: i32, dirp: *mut u8, count: usize);
    syscall!(clock_gettime, CLOCK_GETTIME, clk_id: i32, tp: *mut u8);
    syscall!(clock_nanosleep, CLOCK_NANOSLEEP, clk_id: i32, flags: i32, rqtp: *const u8, rmtp: *mut u8);
    syscall!(epoll_wait, EPOLL_WAIT, epfd: i32, events: *mut u8, maxevents: i32, timeout: i32);
    syscall!(epoll_ctl, EPOLL_CTL, epfd: i32, op: i32, fd: i32, event: *mut u8);
    syscall!(openat, OPENAT, dirfd: i32, pathname: *const u8, flags: i32, mode: u32);
    syscall!(unlinkat, UNLINKAT, dirfd: i32, pathname: *const u8, flags: i32);
    syscall!(splice, SPLICE, fd_in: i32, off_in: *mut i64, fd_out: i32, off_out: *mut i64, len: usize, flags: u32);
    syscall!(tee, TEE, fd_in: i32, fd_out: i32, len: usize, flags: u32);
    syscall!(timerfd_create, TIMERFD_CREATE, clockid: i32, flags: i32);
    syscall!(fallocate, FALLOCATE, fd: i32, mode: i32, offset: usize, len: usize);
    syscall!(timerfd_settime, TIMERFD_SETTIME, fd: i32, flags: i32, new_value: *const u8, old_value: *mut u8);
    syscall!(accept4, ACCEPT4, sockfd: i32, addr: *mut u8, addrlen: *mut u32, flags: i32);
    syscall!(eventfd2, EVENTFD2, initval: u32, flags: i32);
    syscall!(epoll_create1, EPOLL_CREATE1, flags: i32);
    syscall!(pipe2, PIPE2, pipefd: *mut i32, flags: i32);
    syscall!(recvmmsg, RECVMMSG, sockfd: i32, msgvec: *mut u8, vlen: u32, flags: i32, timeout: *const u8);
    syscall!(sendmmsg, SENDMMSG, sockfd: i32, msgvec: *mut u8, vlen: u32, flags: i32);
    syscall!(getcpu, GETCPU, cpu: *mut u32, node: *mut u32, tcache: *mut u8);
    syscall!(seccomp, SECCOMP, operation: u32, flags: u32, args: *const u8);
    syscall!(getrandom, GETRANDOM, buf: *mut u8, buflen: usize, flags: u32);
    syscall!(memfd_create, MEMFD_CREATE, name: *const u8, flags: u32);
    syscall!(bpf, BPF, cmd: i32, attr: *const u8, size: u32);
    syscall!(userfaultfd, USERFAULTFD, flags: i32);
    syscall!(copy_file_range, COPY_FILE_RANGE, fd_in: i32, off_in: *mut i64, fd_out: i32, off_out: *mut i64, len: usize, flags: u32);
    syscall!(pwritev2, PWRITEV2, fd: i32, iov: *const u8, iovcnt: i32, offset: usize, flags: i32);
    syscall!(statx, STATX, dirfd: i32, pathname: *const u8, flags: i32, mask: u32, statxbuf: *mut u8);
    syscall!(io_uring_setup, IO_URING_SETUP, entries: u32, params: *mut u8);
    syscall!(io_uring_enter, IO_URING_ENTER, fd: i32, to_submit: u32, min_complete: u32, flags: u32, sigset: *const u8, sigsetsize: usize);
    syscall!(io_uring_register, IO_URING_REGISTER, fd: i32, opcode: u32, arg: *const u8, nr_args: u32);
    syscall!(pidfd_open, PIDFD_OPEN, pid: i32, flags: u32);
    syscall!(clone3, CLONE3, cl_args: *const u8, size: usize);
    syscall!(futex_waitv, FUTEX_WAITV, waiters: *const u8, nr_futexes: u32, flags: u32, timeout: *const u8, clockid: i32);
}
