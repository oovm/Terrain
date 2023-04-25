use crate::GridTerrain;
use diamond_square::{uniform_area2d, MidpointDisplacement};
use ndarray::Array2;
use rand::{rngs::SmallRng, SeedableRng};
use std::{num::NonZeroU32, ops::Range};

impl From<DiamondSquare> for GridTerrain {
    fn from(value: DiamondSquare) -> Self {
        value.as_terrain()
    }
}

impl From<MidpointDisplacement> for GridTerrain {
    fn from(value: MidpointDisplacement) -> Self {
        GridTerrain::mid_point_displacement(&value)
    }
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
#[derive(Debug)]
pub struct DiamondSquare {
    width: NonZeroU32,
    height: NonZeroU32,
    /// Iteration of the algorithm
    iteration: u32,
    /// Roughness of the grid
    roughness: f32,
    /// Range of the random number generator
    range: Range<f32>,
    /// Seed of the random number generator
    seed: u64,
}

impl Default for DiamondSquare {
    fn default() -> Self {
        unsafe {
            Self {
                width: NonZeroU32::new_unchecked(2),
                height: NonZeroU32::new_unchecked(2),
                iteration: 2,
                seed: 42,
                roughness: 1.1,
                range: Range { start: -1.0, end: 1.0 },
            }
        }
    }
}

impl DiamondSquare {
    pub fn new(width: u32, height: u32) -> Self {
        DiamondSquare::default().with_size(width, height)
    }
    pub fn get_size(&self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }
    pub fn get_map_size(&self) -> (u32, u32) {
        let pow = 2u32.pow(self.iteration);
        (self.width.get() * pow, self.height.get() * pow)
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = NonZeroU32::new(width).expect("Width must be greater than 0");
        self.height = NonZeroU32::new(height).expect("Height must be greater than 0");
    }
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.set_size(width, height);
        self
    }
    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }
    pub fn set_iteration(&mut self, iteration: u32) {
        self.iteration = iteration;
    }
    pub fn with_iteration(mut self, iteration: u32) -> Self {
        self.set_iteration(iteration);
        self
    }
    pub fn get_roughness(&self) -> f32 {
        self.roughness
    }
    pub fn set_roughness(&mut self, roughness: f32) {
        assert!(roughness > 1.0, "Roughness must be greater than 1.0");
        self.roughness = roughness;
    }
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.set_roughness(roughness);
        self
    }
    pub fn as_terrain(&self) -> GridTerrain {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let array = uniform_area2d(
            (self.width.get() as usize, self.height.get() as usize),
            (self.range.start, self.range.end),
            &mut rng,
        );
        let grid = diamond_square::DiamondSquare::new(self.iteration, self.roughness).enlarge(array.view(), &mut rng);
        let min = grid.iter().fold(f32::MAX, |acc, &x| acc.min(x));
        let max = grid.iter().fold(f32::MIN, |acc, &x| acc.max(x));
        GridTerrain { grid, range: min..max }
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
}
