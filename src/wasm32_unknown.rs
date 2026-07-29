//! `wasm32-unknown-unknown` definitions.
//!
//! Experimental support for the WebAssembly memory-control proposal:
//! `madvise(MADV_DONTNEED)` is implemented against an unlinked `discard`
//! import, expected to be provided by the host or replaced by a
//! `memory.discard` trampoline at bindgen time.

use crate::prelude::*;

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_DONTNEED: c_int = 4;

pub const EINVAL: c_int = 22;

extern "C" {
    /// Unlinked import satisfied by a generated `memory.discard` trampoline.
    fn discard(addr: *mut c_void, len: size_t);
}

/// Prototype `madvise` for wasm: `MADV_DONTNEED` discards the pages via
/// the `memory.discard` instruction, other advice is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int {
    match advice {
        MADV_DONTNEED => {
            discard(addr, len);
            0
        }
        _ => 0,
    }
}
