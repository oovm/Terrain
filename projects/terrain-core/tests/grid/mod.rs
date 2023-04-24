use diamond_square::MidpointDisplacement;
use terrain_core::{DiamondSquare, GridTerrain};

#[test]
fn test17x17() {
    let ds = DiamondSquare::default().with_iteration(2);
    let grid = GridTerrain::diamond_square(&ds);
    println!("{:?}", grid);
}

#[test]
fn test17x1() {
    let config = MidpointDisplacement::default().with_iteration(2);
    let grid = GridTerrain::mid_point_displacement(&config);
    println!("{:?}", grid);
}
