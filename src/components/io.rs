// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use super::syscall::*;

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub off: u64,
    pub addr: u64,
    pub len: u32,
    pub rw_flags: u32,
    pub user_data: u64,
    pub buf_index: u16,
    pub personality: u16,
    pub splice_fd_in: i32,
    pub addr3: u64,
    pub __pad2: [u64; 1],
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

#[repr(C)]
pub struct io_sqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[repr(C)]
pub struct io_cqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub flags: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[repr(C)]
pub struct io_uring_params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub features: u32,
    pub wq_fd: u32,
    pub resv: [u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

pub const IORING_OFF_SQ_RING: usize = 0;
pub const IORING_OFF_CQ_RING: usize = 0x8000000;
pub const IORING_OFF_SQES: usize = 0x10000000;

pub const PROT_READ: i32 = 1;
pub const PROT_WRITE: i32 = 2;
pub const MAP_SHARED: i32 = 1;
pub const MAP_POPULATE: i32 = 0x8000;

pub struct IoUring {
    pub fd: i32,
    pub sq_ring: *mut u8,
    pub sq_size: usize,
    pub cq_ring: *mut u8,
    pub cq_size: usize,
    pub sqes: *mut io_uring_sqe,
    pub sqes_size: usize,
}

impl Drop for IoUring {
    fn drop(&mut self) {
        if !self.sqes.is_null() {
            let _ = Syscall::munmap(self.sqes as *mut u8, self.sqes_size);
        }
        if !self.cq_ring.is_null() {
            let _ = Syscall::munmap(self.cq_ring, self.cq_size);
        }
        if !self.sq_ring.is_null() {
            let _ = Syscall::munmap(self.sq_ring, self.sq_size);
        }
        if self.fd >= 0 {
            let _ = Syscall::close(self.fd);
        }
    }
}

impl IoUring {
    #[inline(always)]
    pub fn new(entries: u32) -> Result<Self, i32> {
        let mut params: io_uring_params = unsafe { core::mem::zeroed() };
        let fd = Syscall::io_uring_setup(entries, &mut params as *mut _ as *mut u8)? as i32;

        let sq_size = params.sq_off.array as usize
            + (params.sq_entries as usize * core::mem::size_of::<u32>());
        let cq_size = params.cq_off.cqes as usize
            + (params.cq_entries as usize * core::mem::size_of::<io_uring_cqe>());
        let sqes_size = params.sq_entries as usize * core::mem::size_of::<io_uring_sqe>();

        let mut ring = Self {
            fd,
            sq_ring: core::ptr::null_mut(),
            sq_size,
            cq_ring: core::ptr::null_mut(),
            cq_size,
            sqes: core::ptr::null_mut(),
            sqes_size,
        };

        ring.sq_ring = Syscall::mmap(
            core::ptr::null_mut(),
            sq_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_POPULATE,
            fd,
            IORING_OFF_SQ_RING,
        )? as *mut u8;

        ring.cq_ring = Syscall::mmap(
            core::ptr::null_mut(),
            cq_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_POPULATE,
            fd,
            IORING_OFF_CQ_RING,
        )? as *mut u8;

        ring.sqes = Syscall::mmap(
            core::ptr::null_mut(),
            sqes_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_POPULATE,
            fd,
            IORING_OFF_SQES,
        )? as *mut io_uring_sqe;

        Ok(ring)
    }

    #[inline(always)]
    pub fn enter(&self, to_submit: u32, min_complete: u32, flags: u32) -> Result<usize, i32> {
        Syscall::io_uring_enter(
            self.fd,
            to_submit,
            min_complete,
            flags,
            core::ptr::null(),
            0,
        )
    }
}
