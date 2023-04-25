use ndarray::{Array2, ArrayView2};
use rand::{rngs::SmallRng, Rng, SeedableRng};

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
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiamondSquare {
    /// Iteration of the algorithm
    iteration: u32,
    /// Roughness of the grid
    roughness: f32,
    /// Seed of the random number generator
    seed: u64,
}

impl Default for DiamondSquare {
    fn default() -> Self {
        unsafe { Self { iteration: 2, seed: 42, roughness: 1.1 } }
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
    pub fn enlarge(&self, matrix: ArrayView2<f32>) -> Array2<f32> {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        unsafe {
            let (mut output, w, h) = self.enlarge_map(matrix);
            for iteration in 0..self.iteration {
                println!("Iteration: {}, step: {} in ({}, {})", iteration + 1, step, h, w);
                // diamond step
                let half = step / 2;
                for j in (half..h).step_by(step) {
                    for i in (half..w).step_by(step) {
                        let lu = *output.uget([i - half, j - half]);
                        let ru = *output.uget([(i + half) % w, j - half]);
                        let ld = *output.uget([i - half, (j + half) % h]);
                        let rd = *output.uget([(i + half) % w, (j + half) % h]);
                        *output.uget_mut([i, j]) = self.random_average(&mut rng, [lu, ru, ld, rd]);
                    }
                }
                // square step even rows
                for j in (half..h).step_by(step) {
                    for i in (0..w).step_by(step) {
                        let l = *output.uget([(w + i - half) % w, j]);
                        let r = *output.uget([(0 + i + half), j]);
                        let u = *output.uget([i, (h + j - half) % h]);
                        let d = *output.uget([i, (0 + j + half) % h]);
                        *output.uget_mut([i, j]) = self.random_average(&mut rng, [l, r, u, d]);
                    }
                }
                // square step old rows
                for j in (0..h).step_by(step) {
                    for i in (half..w).step_by(step) {
                        let l = *output.get([i - half, j]).unwrap();
                        let r = *output.get([(i + half) % w, j]).unwrap();
                        let u = *output.get([i, (h + j - half) % h]).unwrap();
                        let d = *output.get([i, (0 + j + half) % h]).unwrap();
                        *output.uget_mut([i, j]) = self.random_average(&mut rng, [l, r, u, d]);
                    }
                }
                step = half;
            }
            // drop last colomn and last row
            output
        }
    }

    unsafe fn enlarge_map(&self, matrix: ArrayView2<f32>) -> (Array2<f32>, usize, usize) {
        let step = 2usize.pow(self.iteration);
        let width = matrix.shape().get_unchecked(0) * step;
        let height = matrix.shape().get_unchecked(1) * step;
        let mut output = Array2::zeros((width, height));
        // fill the corners
        for ((x, y), v) in matrix.indexed_iter() {
            *output.uget_mut((x * step, y * step)) = *v;
        }
        (output, width, height)
    }

    /// Calculate the average of the given values with random multiplier
    fn random_average(&self, rng: &mut SmallRng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        let r_roughness = self.roughness.recip();
        avg * rng.gen_range(r_roughness..self.roughness)
    }
}
