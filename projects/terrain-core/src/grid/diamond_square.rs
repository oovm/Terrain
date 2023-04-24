use super::*;

#[derive(Debug)]
pub struct DiamondSquare {
    pub width: usize,
    pub height: usize,
    pub iteration: u32,
    pub seed: u64,
    pub range: Range<f32>,
}

impl Default for DiamondSquare {
    fn default() -> Self {
        Self {
            width: 4,
            height: 4,
            iteration: 2,
            seed: 0,
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
        let step = 2usize.pow(self.iteration);
        let grid_w = step * self.width;
        let grid_h = step * self.height;
        let mut grid = Array2::zeros((grid_w, grid_h));
        for y in (0..grid_h).step_by(step) {
            for x in (0..grid_w).step_by(step) {
                let value = rng.gen_range(self.range.start..self.range.end);
                grid[[x, y]] = value;
            }
        }
        for iteration in (0..self.iteration).rev() {
            let step_x = self.width * 2usize.pow(iteration);
            let step_y = self.height * 2usize.pow(iteration);
            println!("Iteration: {}, step_x: {}, step_y: {}", iteration, step_x, step_y);
            // Diamond step
            for y in (step_y..grid_h).step_by(step_y) {
                for x in (step_x..grid_w).step_by(step_x) {
                    let tl = grid[[x - step_x, y - step_y]];
                    let tr = grid[[x + step_x, y - step_y]];
                    let bl = grid[[x - step_x, y + step_y]];
                    let br = grid[[x + step_x, y + step_y]];

                    let avg = (tl + tr + bl + br) / 4.0;
                    let value = avg + rng.gen_range(self.range.start..self.range.end);

                    grid[[x, y]] = value;
                }
            }
            // Square step
            for y in (0..grid_h).step_by(step_y) {
                for x in (0..grid_w).step_by(step_x) {
                    let mut sum = 0.0;
                    let mut count = 0;

                    if x >= step_x {
                        sum += grid[[x - step_x, y]];
                        count += 1;
                    }

                    if x + step_x < grid_w {
                        sum += grid[[x + step_x, y]];
                        count += 1;
                    }

                    if y >= step_y {
                        sum += grid[[x, y - step_y]];
                        count += 1;
                    }

                    if y + step_y < grid_h {
                        sum += grid[[x, y + step_y]];
                        count += 1;
                    }

                    let avg = sum / count as f32;
                    let value = avg + rng.gen_range(self.range.start..self.range.end);

                    grid[[x, y]] = value;
                }
            }
        }
        GridTerrain { grid }
    }
}
