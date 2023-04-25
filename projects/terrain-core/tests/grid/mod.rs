use diamond_square::{uniform_area2d, MidpointDisplacement};
use terrain_core::{DiamondSquare, GridTerrain};

#[test]
fn test17x17() {
    let ds = DiamondSquare::default().with_size(20, 20).with_iteration(5).with_seed(rand::random());
    let name = format!("test{}x{}.png", ds.get_map_size().0, ds.get_map_size().1);
    let grid = GridTerrain::diamond_square(&ds);
    grid.as_gray().save(name).unwrap();
}

#[test]
fn test17x1() {
    let config = MidpointDisplacement::default().with_length(10).with_iteration(2).with_seed(rand::random());
    let grid = GridTerrain::mid_point_displacement(&config);
    println!("{:?}", grid);
}

#[test]
fn test() {
    let ds = DiamondSquare::default().with_size(20, 20).with_iteration(1).with_seed(rand::random());
    let mut rng = rand::thread_rng();
    let array = uniform_area2d(4, 2, &mut rng);
    let a = ds.generate_by_array(array.view());
    println!("{:?}", a);
}
