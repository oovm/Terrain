#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![doc = include_str!("../readme.md")]

mod ds;
mod md;

pub use crate::ds::DiamondSquare;

pub use crate::md::MidpointDisplacement;
