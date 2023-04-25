use ndarray::Array2;
use rand::Rng;

pub fn uniform_area2d<R>(width: usize, height: usize, rng: &mut R) -> Array2<f32>
where
    R: Rng,
{
    assert_ne!(width, 0, "width must be greater than 0");
    assert_ne!(height, 0, "height must be greater than 0");
    Array2::from_shape_fn((height, width), |_| rng.gen_range(-1.0..1.0))
}
