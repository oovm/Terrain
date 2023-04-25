use diamond_square::MidpointDisplacement;
use terrain_core::{DiamondSquare, GridTerrain};

#[test]
fn test17x17() {
    let ds = DiamondSquare::new(8, 8).with_iteration(6);
    let name = format!("test{}x{}.png", ds.get_map_size().0, ds.get_map_size().1);
    let grid = ds.as_terrain();
    grid.as_gray().save(name).unwrap();
}

#[test]
fn test17x1() {
    let config = MidpointDisplacement::default().with_length(10).with_iteration(2).with_seed(rand::random());
    let grid = GridTerrain::mid_point_displacement(&config);
    println!("{:?}", grid);
}
