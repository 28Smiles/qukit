#![no_std]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]
#![feature(int_log)]
#![feature(test)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(bench_black_box)]

extern crate typenum;
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod error;
pub mod complex;
pub mod quantum;
#[cfg(feature = "wasm-pack")]
pub mod bindgen;
