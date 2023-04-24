use std::ops::Range;
use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub mod diamond_square;



#[derive(Debug)]
pub struct GridTerrain {
    grid: Array2<f32>,
    range: Range<f32>,
}
