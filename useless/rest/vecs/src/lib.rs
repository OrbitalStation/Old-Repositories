//!
//! Crate for convenient usage of OpenGL shaders-like vectors.
//!
//! Main feature - powerful constructor, for
//! examples check out the `examples` folder.
//!
//!

#![feature(const_trait_impl)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(specialization)]

#![no_std]

#![allow(non_camel_case_types)]
#![allow(incomplete_features)]

mod impls;
mod private;
mod init;
mod alias;

pub(crate) use private::*;

pub use alias::*;

#[derive(Copy, Clone)]
pub struct vec <T: Copy, const N: usize> ([T; N]);
