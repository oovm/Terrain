use ndarray::{Array2, ArrayView2};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{num::NonZeroUsize, ops::Range};

/// Generate a grid using diamond square algorithm
///
/// # Arguments
///
/// * `rng`:
/// * `vs`:
///
/// returns: f32
///
/// # Examples
///
/// ```
/// use ndarray::Array2;
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiamondSquare {
    /// Width of the grid
    width: NonZeroUsize,
    /// Height of the grid
    height: NonZeroUsize,
    /// Iteration of the algorithm
    iteration: u32,
    /// Range of the random number
    range: Range<f32>,
    /// Roughness of the grid
    roughness: f32,
    /// Seed of the random number generator
    seed: u64,
}

impl Default for DiamondSquare {
    fn default() -> Self {
        unsafe {
            Self {
                width: NonZeroUsize::new_unchecked(4),
                height: NonZeroUsize::new_unchecked(4),
                iteration: 2,
                seed: 42,
                roughness: 1.1,
                range: Range { start: -1.0, end: 1.0 },
            }
        }
    }
}

impl DiamondSquare {
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_size(), (4, 4));
    /// ```
    pub fn get_size(&self) -> (usize, usize) {
        (self.width.get(), self.height.get())
    }
    /// Get the final size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_map_size(), (17, 17));
    /// ```
    pub fn get_map_size(&self) -> (usize, usize) {
        let step = 2usize.pow(self.iteration);
        let w = step * self.width.get() + 1;
        let h = step * self.height.get() + 1;
        (w, h)
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// cfg.set_size(17, 17);
    /// assert_eq!(cfg.get_size(), (17, 17));
    /// ```
    pub fn set_size(&mut self, width: usize, height: usize) {
        assert!(width > 0, "width must be greater than 0");
        assert!(height > 0, "height must be greater than 0");
        unsafe {
            self.width = NonZeroUsize::new_unchecked(width);
            self.height = NonZeroUsize::new_unchecked(height);
        }
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default().with_size(17, 17);
    /// assert_eq!(cfg.get_size(), (17, 17));
    /// ```
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.set_size(width, height);
        self
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_iteration(), 2);
    /// ```
    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// cfg.set_iteration(5);
    /// assert_eq!(cfg.get_iteration(), 5);
    /// ```
    pub fn set_iteration(&mut self, iteration: u32) {
        assert!(iteration < 30, "iteration too high, out of memory");
        self.iteration = iteration;
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default().with_iteration(5);
    /// assert_eq!(cfg.get_iteration(), 5);
    /// ```
    pub fn with_iteration(mut self, iteration: u32) -> Self {
        self.set_iteration(iteration);
        self
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_range().start, -1.0);
    /// assert_eq!(cfg.get_range().end, 1.0);
    /// ```
    pub fn get_range(&self) -> Range<f32> {
        self.range.clone()
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// cfg.set_range(-2.0..2.0);
    /// assert_eq!(cfg.get_range().start, -2.0);
    /// ```
    pub fn set_range(&mut self, range: Range<f32>) {
        self.range = range;
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default().with_range(-2.0..2.0);
    /// assert_eq!(cfg.get_range().start, -2.0);
    /// assert_eq!(cfg.get_range().end, 2.0);
    /// ```
    pub fn with_range(mut self, range: Range<f32>) -> Self {
        self.set_range(range);
        self
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_roughness(), 1.1);
    /// ```
    pub fn get_roughness(&self) -> f32 {
        self.roughness
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// cfg.set_roughness(1.5);
    /// assert_eq!(cfg.get_roughness(), 1.5);
    /// ```
    pub fn set_roughness(&mut self, roughness: f32) {
        assert!(roughness >= 1.0, "roughness must be greater than 1.0");
        self.roughness = roughness;
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default().with_roughness(1.5);
    /// assert_eq!(cfg.get_roughness(), 1.5);
    /// ```
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.set_roughness(roughness);
        self
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// assert_eq!(cfg.get_seed(), 42);
    /// ```
    pub fn get_seed(&self) -> u64 {
        self.seed
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default();
    /// cfg.set_seed(0);
    /// assert_eq!(cfg.get_seed(), 0);
    /// ```
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut cfg = DiamondSquare::default().with_seed(0);
    /// assert_eq!(cfg.get_seed(), 0);
    /// ```
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.set_seed(seed);
        self
    }
}

impl DiamondSquare {
    /// Generate a grid using diamond square algorithm
    ///
    /// # Arguments
    ///
    /// * `rng`:
    /// * `vs`:
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array2;
    /// ```
    pub fn generate(&self) -> Array2<f32> {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        let (w, h) = self.get_map_size();
        let mut grid = Array2::zeros((w, h));
        for x in (0..w).step_by(step) {
            for y in (0..h).step_by(step) {
                // SAFETY: obviously x and y are in bounds
                unsafe {
                    *grid.uget_mut((x, y)) = rng.gen_range(self.range.clone());
                }
            }
        }
        for iteration in 0..self.iteration {
            tracing::trace!("Iteration: {}, step: {} in ({}, {})", iteration + 1, step, w, h);
            // diamond step
            let half = step / 2;
            for i in (half..h).step_by(step) {
                for j in (half..w).step_by(step) {
                    let lu = grid[[i - half, j - half]];
                    let ru = grid[[i - half, j + half]];
                    let ld = grid[[i + half, j - half]];
                    let rd = grid[[i + half, j + half]];
                    grid[[i, j]] = self.random_average(&mut rng, [lu, ru, ld, rd]);
                }
            }
            // square step even rows
            for i in (half..w).step_by(step) {
                for j in (0..h).step_by(step) {
                    let l = grid[[i - half, j]];
                    let r = grid[[i + half, j]];
                    let u = grid[[i, (h + j - half) % h]];
                    let d = grid[[i, (0 + j + half) % h]];
                    grid[[i, j]] = self.random_average(&mut rng, [l, r, u, d]);
                }
            }
            // square step old rows
            for i in (0..w).step_by(step) {
                for j in (half..h).step_by(step) {
                    let l = grid[[(w + i - half) % w, j]];
                    let r = grid[[(0 + i + half) % w, j]];
                    let u = grid[[i, j - half]];
                    let d = grid[[i, j + half]];
                    grid[[i, j]] = self.random_average(&mut rng, [l, r, u, d]);
                }
            }
            step /= 2;
        }
        grid
    }
    unsafe fn enlarge_map(&self, matrix: ArrayView2<f32>) -> Array2<f32> {
        let mut step = 2usize.pow(self.iteration);
        let w = matrix.shape()[0] * step + 1;
        let h = matrix.shape()[1] * step + 1;
        let mut output = Array2::zeros((w, h));
        // fill the corners
        for ((x, y), v) in matrix.indexed_iter() {
            *output.uget_mut((x * step, y * step)) = *v;
        }
        // last line is first line,
        for x in 0..w {
            *output.uget_mut((x, h - 1)) = *output.uget((x, 0));
        }
        // last row is first row
        for y in 0..h {
            output[[w - 1, y]] = output[[0, y]];
        }
        output
    }

    pub fn generate_by_array(&self, matrix: ArrayView2<f32>) -> Array2<f32> {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        let w = matrix.shape()[0] * step + 1;
        let h = matrix.shape()[1] * step + 1;
        let mut output = Array2::zeros((w, h));
        // fill the corners
        for ((x, y), v) in matrix.indexed_iter() {
            // SAFETY: obviously x and y are in bounds
            unsafe {
                *output.uget_mut((x * step, y * step)) = *v;
            }
        }
        // last line is first line, last row is first row
        for x in 0..w {
            output[[x, h - 1]] = output[[x, 0]];
        }
        for y in 0..h {
            output[[w - 1, y]] = output[[0, y]];
        }
        for iteration in 0..self.iteration {
            println!("Iteration: {}, step: {} in ({}, {})", iteration + 1, step, w, h);
            // diamond step
            let half = step / 2;
            for i in (half..h).step_by(step) {
                for j in (half..w).step_by(step) {
                    // let lu = output[[i - half, j - half]];
                    // let ru = output[[i - half, j + half]];
                    // let ld = output[[i + half, j - half]];
                    // let rd = output[[i + half, j + half]];
                    // output[[i, j]] = self.random_average(&mut rng, [lu, ru, ld, rd]);
                }
            }
        }

        output
    }
    /// Calculate the average of the given values with random multiplier
    fn random_average(&self, rng: &mut SmallRng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        let r_roughness = self.roughness.recip();
        avg * rng.gen_range(r_roughness..self.roughness)
    }
}
