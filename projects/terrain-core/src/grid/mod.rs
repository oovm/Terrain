use crate::{TerrainError, TerrainResult};
use image::GrayImage;
use ndarray::Array2;
use std::ops::{Index, Range};

pub mod diamond_square;
mod export_image;

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

impl GridTerrain {
    pub fn get_size(&self) -> (usize, usize) {
        let shape = self.grid.shape();
        // SAFETY: shape have exactly 2 elements
        unsafe { (*shape.get_unchecked(0), *shape.get_unchecked(1)) }
    }
    pub fn get_width(&self) -> usize {
        self.get_size().0
    }
    pub fn get_height(&self) -> usize {
        self.get_size().1
    }
    pub fn set_min(&mut self, min: f32) -> TerrainResult<()> {
        if min < self.range.end {
            self.range.start = min;
            Ok(())
        }
        else {
            Err(TerrainError::invalid_range(format!(
                "new minimum height {} does not lower than current minimum height {}",
                min, self.range.end
            )))
        }
    }
    pub fn set_max(&mut self, max: f32) -> TerrainResult<()> {
        if max > self.range.start {
            self.range.end = max;
            Ok(())
        }
        else {
            Err(TerrainError::invalid_range(format!(
                "new maximum height {} does not higher than current maximum height {}",
                max, self.range.start
            )))
        }
    }
    /// Normalize a value to range [0, 1]
    pub fn get_normed(&self, value: f32) -> f32 {
        (value - self.range.start) / (self.range.end - self.range.start)
    }
    /// Normalize a value to range [0, 1]
    pub fn map_height<F>(&mut self, f: F)
    where
        F: Fn(f32) -> f32,
    {
        self.grid.iter_mut().for_each(|x| *x = f(*x));
    }
}

impl Index<(u32, u32)> for GridTerrain {
    type Output = f32;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        debug_assert!(
            x < self.get_width() as u32 && y < self.get_height() as u32,
            "({}, {}) is out of bound ({}, {})",
            x,
            y,
            self.get_width(),
            self.get_height()
        );
        // SAFETY: x and y are less than width and height
        unsafe { self.grid.uget([x as usize, y as usize]) }
    }
}
