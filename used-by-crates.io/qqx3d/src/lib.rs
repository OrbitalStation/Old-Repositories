#![allow(incomplete_features)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(trait_alias)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

mod window;
mod vec;
mod color;
mod polygon;
mod event;

pub(crate) struct Stt;

pub extern crate ctor;
pub extern crate glium;

pub use qqx_macro::qqx;

pub use vec::{Vec1, Vec2, Vec3, Vec4};
pub use color::Color;
pub use window::{Window, Drawable};
pub use polygon::{Polygon, BoundPolygonInterface, BoundPolygonInterfaceAction};
pub use event::{callback, eventloop, ControlFlow};
