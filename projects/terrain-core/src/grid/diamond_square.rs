use rand::distributions::uniform::SampleRange;
use super::*;

#[derive(Debug)]
pub struct DiamondSquare {
    pub width: usize,
    pub height: usize,
    pub iteration: u32,
    pub range: Range<f32>,
    pub roughness: f32,
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
    pub fn generate(&self) -> GridTerrain {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut step = 2usize.pow(self.iteration);
        let w = step * self.width + 1;
        let h = step * self.height + 1;
        let mut grid = Array2::zeros((w, h));
        let mut range = self.range.clone();
        for x in (0..h).step_by(step) {
            for y in (0..w).step_by(step) {
                let value = rng.gen_range(self.range.start..self.range.end);
                range.start = range.start.min(value);
                range.end = range.end.max(value);
                grid[[x, y]] = value;
            }
        }
        for iteration in 0..self.iteration {
            println!("Iteration: {}, step: {} in ({}, {})", iteration + 1, step, w, h);
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
        GridTerrain { grid, range }
    }

    pub fn random_average(&self, rng: &mut SmallRng, vs: [f32; 4]) -> f32 {
        let avg = vs.iter().sum::<f32>() / 4.0;
        let r_roughness = self.roughness.recip();
        avg * rng.gen_range(r_roughness..self.roughness)
    }
}
