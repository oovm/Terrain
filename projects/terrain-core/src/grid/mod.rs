use std::ops::Range;
use ndarray::Array2;

pub struct DiamondSquare {
    pub width: usize,
    pub height: usize,
    pub iteration: usize,
    pub seed: u64,
    pub range: Range<f32>,
}

pub struct GridTerrain {
    grid: Array2<f64>,
}

impl DiamondSquare {
    pub fn generate(&self) -> GridTerrain {
        let mut grid = Array2::zeros((self.width, self.height));
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        let mut range = self.range.clone();

        let mut x = 0;
        let mut y = 0;
        let mut w = self.width;
        let mut h = self.height;

        for _ in 0..self.iteration {
            let half_w = w / 2;
            let half_h = h / 2;

            // Diamond step
            for y in (half_h..self.height).step_by(h) {
                for x in (half_w..self.width).step_by(w) {
                    let tl = grid[[x - half_w, y - half_h]];
                    let tr = grid[[x + half_w, y - half_h]];
                    let bl = grid[[x - half_w, y + half_h]];
                    let br = grid[[x + half_w, y + half_h]];

                    let avg = (tl + tr + bl + br) / 4.0;
                    let value = avg + range.ind_sample(&mut rng);

                    grid[[x, y]] = value;
                }
            }

            // Square step
            for y in (0..self.height).step_by(half_h) {
                for x in (0..self.width).step_by(half_w) {
                    let mut sum = 0.0;
                    let mut count = 0;

                    if x >= half_w {
                        sum += grid[[x - half_w, y]];
                        count += 1;
                    }

                    if x + half_w < self.width {
                        sum += grid[[x + half_w, y]];
                        count += 1;
                    }

                    if y >= half_h {
                        sum += grid[[x, y - half_h]];
                        count += 1;
                    }

                    if y + half_h < self.height {
                        sum += grid[[x, y + half_h]];
                        count += 1;
                    }

                    let avg = sum / count as f32;
                    let value = avg + range.ind_sample(&mut rng);

                    grid[[x, y]] = value;
                }
            }

            w /= 2;
            h /= 2;
            range.start *= 0.5;
            range.end *= 0.5;
        }
        GridTerrain { grid }
    }
}

