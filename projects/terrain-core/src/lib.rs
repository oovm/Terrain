#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![doc = include_str!("../readme.md")]

mod errors;

mod grid;

pub use errors::{TerrainError, TerrainErrorKind, TerrainResult};

pub use crate::grid::{diamond_square::DiamondSquare, GridTerrain};
