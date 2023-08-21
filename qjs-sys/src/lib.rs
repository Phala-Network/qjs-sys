#![no_std]
extern crate alloc;

mod alloc_impl;

pub mod c;
#[cfg_attr(target_pointer_width = "32", path = "inline32.rs")]
#[cfg_attr(target_pointer_width = "64", path = "inline64.rs")]
pub mod inline_fns;
mod libc {
    pub use core::ffi::*;
}