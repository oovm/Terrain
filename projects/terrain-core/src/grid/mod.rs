use std::ops::Range;
use ndarray::Array2;


#[derive(Debug)]
pub struct GridTerrain {
    grid: Array2<f32>,
    range: Range<f32>,
}

mod diamond_square;