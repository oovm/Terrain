use ndarray::Array2;
use std::ops::Range;

mod diamond_square;

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
#[derive(Debug)]
pub struct GridTerrain {
    grid: Array2<f32>,
    range: Range<f32>,
}
