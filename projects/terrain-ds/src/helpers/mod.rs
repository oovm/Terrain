use ndarray::Array2;
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
/// # use diamond_square::DiamondSquare;
/// let mut cfg = DiamondSquare::default();
/// assert_eq!(cfg.get_iteration(), 2);
/// ```
pub fn uniform_area2d<R>((width, height): (usize, usize), (min, max): (f32, f32), rng: &mut R) -> Array2<f32>
where
    R: Rng,
{
    debug_assert_ne!(width, 0, "width must be greater than 0");
    debug_assert_ne!(height, 0, "height must be greater than 0");
    debug_assert!(min < max, "min must be less than max");
    Array2::from_shape_fn((height, width), |_| rng.gen_range(min..max))
}
