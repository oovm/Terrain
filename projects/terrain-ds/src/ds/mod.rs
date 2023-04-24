use std::ops::Range;
use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};


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
    pub width: usize,
    /// Height of the grid
    pub height: usize,
    /// Iteration of the algorithm
    pub iteration: u32,
    /// Range of the random number
    pub range: Range<f32>,
    /// Roughness of the grid
    pub roughness: f32,
    /// Seed of the random number generator
    pub seed: u64,
}

impl Default for DiamondSquare {
    fn default() -> Self {
        Self {
            width: 4,
            height: 4,
            iteration: 2,
            seed: 0,
            roughness: 1.1,
            range: Range {
                start: -1.0,
                end: 1.0,
            },
        }
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
        let w = step * self.width + 1;
        let h = step * self.height + 1;
        let mut grid = Array2::zeros((w, h));
        for x in (0..h).step_by(step) {
            for y in (0..w).step_by(step) {
                let value = rng.gen_range(self.range.start..self.range.end);
                grid[[x, y]] = value;
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

    fn random_average(&self, rng: &mut SmallRng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        let r_roughness = self.roughness.recip();
        avg * rng.gen_range(r_roughness..self.roughness)
    }
}
