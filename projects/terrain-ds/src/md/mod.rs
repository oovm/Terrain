use ndarray::{Array1, Array2};
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

impl MidpointDisplacement {
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut ds = DiamondSquare::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn get_length(&self) -> usize {
        self.length.get()
    }
    /// Get the initial size of the terrain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use diamond_square::DiamondSquare;
    /// let mut ds = DiamondSquare::default();
    /// assert_eq!(ds.get_size(), (4, 4));
    /// ```
    pub fn get_map_length(&self) -> usize {
        let mut step = 2usize.pow(self.iteration);
        self.length.get() * step + 1
    }
}

impl MidpointDisplacement {
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
    pub fn generate(&self) -> Array1<f32> {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        let w = self.get_map_size();
        let mut grid = Array1::zeros((w, h));
        for x in (0..w).step_by(step) {}
        for iteration in 0..self.iteration {
            tracing::trace!("Iteration: {}, step: {} in {}", iteration + 1, step, w, h);
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
            step /= 2;
        }
        grid
    }
    /// Calculate the average of the given values with random multiplier
    fn random_average(&self, rng: &mut SmallRng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        let r_roughness = self.roughness.recip();
        avg * rng.gen_range(r_roughness..self.roughness)
    }
}
