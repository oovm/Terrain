mod errors;

mod grid;

pub use errors::{Error, Result};



pub use crate::grid::{GridTerrain, diamond_square::DiamondSquare};