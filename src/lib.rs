#![no_std]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]
#![feature(int_log)]
#![feature(test)]
#![feature(core_intrinsics)]
#![feature(extern_types)]

extern crate alloc;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature="console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub mod error;
pub mod complex;
pub mod quantum;
#[cfg(feature = "wasm-pack")]
pub mod bindgen;
