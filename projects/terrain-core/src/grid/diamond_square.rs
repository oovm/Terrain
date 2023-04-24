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
        let grid_w = step * self.width;
        let grid_h = step * self.height;
        let mut grid = Array2::zeros((grid_w, grid_h));
        let mut range = self.range.clone();
        for x in (0..grid_h).step_by(step) {
            for y in (0..grid_w).step_by(step) {
                let value = rng.gen_range(self.range.start..self.range.end);
                range.start = range.start.min(value);
                range.end = range.end.max(value);
                grid[[x, y]] = value;
            }
        }
        for iteration in 0..self.iteration {
            println!("Iteration: {}, step: {}", iteration + 1, step);
            // Diamond step
            for i in (0..grid_h).step_by(step).map(|i| i.saturating_sub(1)) {
                for j in (0..grid_w).step_by(step).map(|j| j.saturating_sub(1)) {
                    let lu = grid[[i, j]];
                    let ru = grid[[i, j + step]];
                    let ld = grid[[i + step, j]];
                    let rd = grid[[i + step, j + step]];
                    let avg = (lu + ru + ld + rd) / 4.0;
                    let value = avg + self.get_rough_rate(&mut rng);
                    grid[[i + step / 2, j + step / 2]] = value;
                }
            }
            // Square step rows
            for i in (0..grid_h).step_by(step).map(|i| i.saturating_sub(1)) {
                for j in (0..grid_w).step_by(step).map(|j| j.saturating_sub(1)) {
                    let l = grid[[i, j]];
                    let r = grid[[i, j + step % grid_w]];
                    let u = grid[[i + step % grid_h, j]];
                    let d = grid[[i + step % grid_h, j + step % grid_w]];
                    let avg = (l + r + u + d) / 4.0;
                    let value = avg + self.get_rough_rate(&mut rng);
                    grid[[i + step / 2, j]] = value;
                }
            }
            step /= 2;
        }
        GridTerrain { grid, range }
    }

    pub fn get_rough_rate(&self, rng: &mut SmallRng) -> f32 {
        let r_roughness = self.roughness.recip();
        rng.gen_range(r_roughness..self.roughness)
    }
    pub fn is_corner(&self, i: usize, j: usize) -> bool {
        let mut step = 2usize.pow(self.iteration);
        let grid_w = step * self.width;
        let grid_h = step * self.height;
        (i == 0 && j == 0)
            || (i == 0 && j == grid_w - 1)
            || (i == grid_h - 1 && j == 0)
            || (i == grid_h - 1 && j == grid_w - 1)
    }
}
