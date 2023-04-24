use ndarray::Array1;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{num::NonZeroUsize, ops::Range};

/// Generate a grid using diamond square algorithm
///
/// # Examples
///
/// ```
/// use ndarray::Array2;
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MidpointDisplacement {
    /// Length of the segment
    length: NonZeroUsize,
    /// Iteration of the algorithm
    iteration: u32,
    /// Range of the random number
    range: Range<f32>,
    /// Roughness of the grid
    roughness: f32,
    /// Seed of the random number generator
    seed: u64,
}

impl Default for MidpointDisplacement {
    fn default() -> Self {
        unsafe {
            Self {
                length: NonZeroUsize::new_unchecked(4),
                iteration: 2,
                roughness: 1.1,
                seed: 0,
                range: Range { start: -1.0, end: 1.0 },
            }
        }
    }
}

impl MidpointDisplacement {
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_length(), 4);
    /// ```
    pub fn get_length(&self) -> usize {
        self.length.get()
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_map_length(), 17);
    /// ```
    pub fn get_map_length(&self) -> usize {
        let step = 2usize.pow(self.iteration);
        self.length.get() * step + 1
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// ds.set_length(8);
    /// assert_eq!(ds.get_length(), 8);
    /// ```
    pub fn set_length(&mut self, length: usize) {
        self.length = NonZeroUsize::new(length).unwrap();
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn with_length(mut self, length: usize) -> Self {
        self.set_length(length);
        self
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn set_iteration(&mut self, iteration: u32) {
        self.iteration = iteration;
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn with_iteration(mut self, iteration: u32) -> Self {
        self.set_iteration(iteration);
        self
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn set_range(&mut self, range: Range<f32>) {
        self.range = range;
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn with_range(mut self, range: Range<f32>) -> Self {
        self.set_range(range);
        self
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn set_roughness(&mut self, roughness: f32) {
        self.roughness = roughness;
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.set_roughness(roughness);
        self
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::MidpointDisplacement;
    /// let mut ds = MidpointDisplacement::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.set_seed(seed);
        self
    }
}

impl MidpointDisplacement {
    /// Generate a grid using diamond square algorithm
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array2;
    /// ```
    pub fn generate(&self) -> Array1<f32> {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        let length = self.get_map_length();
        let mut grid = Array1::zeros((length,));
        for x in (0..length).step_by(step) {
            grid[[x]] = rng.gen_range(self.range.clone());
        }
        for iteration in 0..self.iteration {
            tracing::trace!("Iteration: {}, step: {} in {}", iteration + 1, step, length);
            // diamond step
            let half = step / 2;
            for i in (half..length).step_by(step) {
                let l = grid[[i - half]];
                let r = grid[[i + half]];
                grid[[i]] = self.random_average(&mut rng, [l, r]);
            }
            step /= 2;
        }
        grid
    }
    /// Calculate the average of the given values with random multiplier
    fn random_average(&self, rng: &mut SmallRng, vs: [f32; 2]) -> f32 {
        let mut avg = vs.iter().sum::<f32>() / 2.0;
        if self.roughness != 1.0 {
            let r_roughness = self.roughness.recip();
            avg *= rng.gen_range(r_roughness..self.roughness);
        }
        avg
    }
}
