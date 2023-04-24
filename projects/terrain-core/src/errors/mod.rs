use std::{
    error::Error,
    fmt::{Display, Formatter},
};
mod display;

/// Generate a grid using diamond square algorithm
///
/// # Arguments
///
/// * `config`:
///
/// returns: GridTerrain
///
/// # Examples
///
/// ```
/// use diamond_square::DiamondSquare;
/// ```
pub type TerrainResult<T> = Result<T, TerrainError>;

/// Generate a grid using diamond square algorithm
///
/// # Arguments
///
/// * `config`:
///
/// returns: GridTerrain
///
/// # Examples
///
/// ```
/// use diamond_square::DiamondSquare;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainError {
    kind: Box<TerrainErrorKind>,
}

/// Generate a grid using diamond square algorithm
///
/// # Arguments
///
/// * `config`:
///
/// returns: GridTerrain
///
/// # Examples
///
/// ```
/// use diamond_square::DiamondSquare;
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TerrainErrorKind {
    /// Unknown error
    UnknownError,
}
