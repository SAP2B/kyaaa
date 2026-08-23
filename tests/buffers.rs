// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::Buffer;

#[test]
fn ring_buffer_fifo() {
    let mut raw_buf = Buffer::<512>::zero();
    let mut ring = raw_buf.fifo();
    ring.push(42).unwrap();
    ring.push(100).unwrap();
    assert_eq!(ring.pop(), Some(42));
    assert_eq!(ring.pop(), Some(100));
    assert_eq!(ring.pop(), None);
}

#[test]
fn stack_buffer_lifo() {
    let mut raw_buf = Buffer::<512>::zero();
    let mut stack = raw_buf.lifo();
    stack.push(42).unwrap();
    stack.push(100).unwrap();
    assert_eq!(stack.pop(), Some(100));
    assert_eq!(stack.pop(), Some(42));
    assert_eq!(stack.pop(), None);
}

#[test]
fn arena_allocator() {
    let mut raw_buf = Buffer::<100>::zero();
    let mut arena = raw_buf.arena();
    assert_eq!(arena.remaining(), 100);
    let chunk1 = arena.alloc(40).unwrap();
    assert_eq!(chunk1.len(), 40);
    assert_eq!(arena.remaining(), 60);
    let chunk2 = arena.alloc(50).unwrap();
    assert_eq!(chunk2.len(), 50);
    assert_eq!(arena.remaining(), 10);
    assert!(arena.alloc(20).is_none());
    arena.reset();
    assert_eq!(arena.remaining(), 100);
    assert!(arena.alloc(20).is_some());
}
