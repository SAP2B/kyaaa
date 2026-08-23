// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_std]

pub mod components;
pub use components::book::Book;
pub use components::buffer::Buffer;

pub mod prelude {
    pub use crate::components::prelude::*;
}
