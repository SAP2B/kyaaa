// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use super::syscall::Syscall;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Thread(pub usize);

impl Thread {
    #[inline(always)]
    pub fn pin(&self) -> Result<usize, i32> {
        let mask: usize = 1 << self.0;
        Syscall::sched_setaffinity(0, core::mem::size_of::<usize>(), &mask)
    }
}
