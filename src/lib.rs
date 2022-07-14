#![no_std]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_for)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_intoiterator_identity)]
#![feature(const_refs_to_cell)]
#![feature(const_default_impls)]

extern crate typenum;
extern crate alloc;

#[cfg(feature = "wasm-bindgen-rayon")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod error;
pub mod api;
pub mod complex;
pub(crate) mod util;
pub(crate) mod runtime;
pub(crate) mod toolbox;
