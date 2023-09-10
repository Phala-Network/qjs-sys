#![no_std]
extern crate alloc;

mod alloc_impl;

pub mod c;

#[allow(clippy::arithmetic_side_effects)]
#[allow(clippy::useless_transmute)]
#[allow(clippy::needless_return)]
#[allow(clippy::let_and_return)]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::single_match)]
#[cfg_attr(target_pointer_width = "32", path = "inline32.rs")]
#[cfg_attr(target_pointer_width = "64", path = "inline64.rs")]
pub mod inline_fns;
mod libc {
    pub use core::ffi::*;
}
