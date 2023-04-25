#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![doc = include_str!("../readme.md")]

mod ds;
mod helpers;
mod md;
pub use crate::ds::DiamondSquare;

pub use crate::{helpers::uniform_area2d, md::MidpointDisplacement};
