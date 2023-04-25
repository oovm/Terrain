use ndarray::{Array2, ArrayView2};
use rand::Rng;

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
}

impl Default for DiamondSquare {
    fn default() -> Self {
        Self { iteration: 2, roughness: 1.1 }
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
    pub fn new(iteration: u32, roughness: f32) -> Self {
        Self { iteration, roughness }
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
    pub fn enlarge(&self, matrix: ArrayView2<f32>, rng: &mut impl Rng) -> Array2<f32> {
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
                        *output.uget_mut([i, j]) = self.random_average(rng, [lu, ru, ld, rd]);
                    }
                }
                // square step even rows
                for j in (half..h).step_by(step) {
                    for i in (0..w).step_by(step) {
                        let l = *output.uget([(w + i - half) % w, j]);
                        let r = *output.uget([(0 + i + half), j]);
                        let u = *output.uget([i, (h + j - half) % h]);
                        let d = *output.uget([i, (0 + j + half) % h]);
                        *output.uget_mut([i, j]) = self.random_average(rng, [l, r, u, d]);
                    }
                }
                // square step old rows
                for j in (0..h).step_by(step) {
                    for i in (half..w).step_by(step) {
                        let l = *output.uget([i - half, j]);
                        let r = *output.uget([(i + half) % w, j]);
                        let u = *output.uget([i, (h + j - half) % h]);
                        let d = *output.uget([i, (0 + j + half) % h]);
                        *output.uget_mut([i, j]) = self.random_average(rng, [l, r, u, d]);
                    }
                }
                step = half;
            }
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
    fn random_average(&self, rng: &mut impl Rng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        if self.roughness == 1.0 {
            avg
        }
        else {
            let r_roughness = self.roughness.recip();
            avg * rng.gen_range(r_roughness..self.roughness)
        }
    }
}
