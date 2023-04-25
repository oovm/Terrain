use crate::{DiamondSquare, GridTerrain};
use diamond_square::{uniform_area2d, MidpointDisplacement};
use ndarray::Array2;

impl From<DiamondSquare> for GridTerrain {
    fn from(value: DiamondSquare) -> Self {
        GridTerrain::diamond_square(&value)
    }
}

impl From<MidpointDisplacement> for GridTerrain {
    fn from(value: MidpointDisplacement) -> Self {
        GridTerrain::mid_point_displacement(&value)
    }
}

impl GridTerrain {
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
    pub fn mid_point_displacement(config: &MidpointDisplacement) -> Self {
        let row0 = config.generate();
        let grid = Array2::from_shape_fn((row0.len(), 1), |(x, _)| row0[x]);
        let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        GridTerrain { grid, range: min..max }
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
    pub fn diamond_square(config: &DiamondSquare) -> Self {
        // let mut grid = uniform_area2d(config.get_width(), config.get_height(), 0.0);
        let grid = config.enlarge(todo!());
        let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        GridTerrain { grid, range: min..max }
    }
}
